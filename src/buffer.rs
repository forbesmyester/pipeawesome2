use crate::motion::PullJourney;
use crate::connectable::Breakable;
use crate::motion::Journey;
use crate::connectable::OutputPort;
use crate::connectable::ConnectableAddOutputError;
use crate::connectable::ConnectableAddInputError;
use crate::connectable::Connectable;
use async_std::{channel::SendError, prelude::*};

use async_std::channel::{bounded, unbounded, Receiver, Sender };
use crate::motion::{MotionNotifications};

use super::motion::{ motion, MotionError, MonitorMessage, MotionResult, Pull, Push, };
use crate::back_off::BackOff;

use crate::startable_control::StartableControl;
use async_trait::async_trait;

#[derive(PartialEq,Debug)]
pub struct BufferSizeMessage(pub usize);

pub struct Buffer {
    stdout_size: usize,
    stdout: Option<Push>,
    stdin: Option<Pull>,
    buffer_size_monitor: Option<Sender<BufferSizeMessage>>,
}


impl Connectable for Buffer {

    fn add_output(&mut self, port: OutputPort, breakable: Breakable, src_id: usize, dst_id: usize) -> std::result::Result<Pull, ConnectableAddOutputError> {
        if self.stdout.is_some() { return Err(ConnectableAddOutputError::AlreadyAllocated(port)); }
        let (child_stdout_push_channel, stdout_io_reciever_channel) = bounded(self.stdout_size);
        self.stdout = Some(Push::IoSender(Journey { src: src_id, dst: dst_id, breakable: breakable.clone() }, child_stdout_push_channel));
        Ok(Pull::Receiver(PullJourney { src: src_id, dst: dst_id }, stdout_io_reciever_channel))
    }

    fn add_input(&mut self, pull: Pull, unused_priority: isize) -> std::result::Result<(), ConnectableAddInputError> {
        if unused_priority != 0 {
            return Err(ConnectableAddInputError::UnsupportedPriority(unused_priority));
        }
        if self.stdin.is_some() {
            return Err(ConnectableAddInputError::AlreadyAllocated);
        }
        self.stdin = Some(pull);
        Ok(())
    }

}


#[allow(clippy::new_without_default)]
impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            stdout_size: 8,
            stdin: None,
            stdout: None,
            buffer_size_monitor: None,
        }
    }

    pub fn add_buffer_size_monitor(&mut self) -> Receiver<BufferSizeMessage> {
        assert!(self.buffer_size_monitor.is_none(), "Each buffer can only be monitored once");
        let (tx, rx) = bounded(self.stdout_size);
        self.buffer_size_monitor = Some(tx);
        rx
    }

    pub fn set_stdout_size(&mut self, size: usize) {
        self.stdout_size = size;
    }

}


#[async_trait]
impl StartableControl for Buffer {
    async fn start(&mut self) -> MotionResult<usize> {

        let (unbounded_snd, unbounded_rcv) = unbounded();
        let (monitor_i_snd, monitor_i_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
        let (monitor_o_snd, monitor_o_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);

        let stdin = std::mem::take(&mut self.stdin).ok_or(MotionError::NoneError)?;
        let stdout = std::mem::take(&mut self.stdout).ok_or(MotionError::NoneError)?;

        let push_a = Push::Sender(
            Journey { src: 0, dst: 0, breakable: stdout.journey().ok_or(MotionError::NoneError)?.breakable.clone()},
            unbounded_snd
        );
        let pull_b = Pull::Receiver(
            PullJourney { src: 0, dst: 0 },
            unbounded_rcv
        );

        let r_a = motion(
            stdin,
            MotionNotifications::written(monitor_i_snd),
            push_a
        );
        let r_b = motion(
            pull_b,
            MotionNotifications::read(monitor_o_snd),
            stdout,
        );

        async fn total_in_buffer(sender: Option<Sender<BufferSizeMessage>>, m_in: Receiver<MonitorMessage>, m_out: Receiver<MonitorMessage>) -> Result<usize, SendError<BufferSizeMessage>> {
            let mut size: usize = 0;
            let mut back_off = BackOff::new();
            let mut last = 0;
            loop {
                let mut buffer_movement = false;
                match m_in.try_recv() {
                    Err(async_std::channel::TryRecvError::Empty) => {
                    },
                    Err(async_std::channel::TryRecvError::Closed) => (),
                    Ok(MonitorMessage::Wrote(_)) => {
                        // println!("+BUF");
                        buffer_movement = true;
                        size += 1;
                    }
                    Ok(MonitorMessage::Read(_)) => {
                        panic!("SHOULD NOT BE HERE");
                    }
                }
                match m_out.try_recv() {
                    Err(async_std::channel::TryRecvError::Empty) => {
                    },
                    Err(async_std::channel::TryRecvError::Closed) => (),
                    Ok(MonitorMessage::Read(_)) => {
                        // println!("-BUF");
                        buffer_movement = true;
                        size -= 1;
                    }
                    Ok(MonitorMessage::Wrote(_)) => {
                        panic!("SHOULD NOT BE HERE");
                    }
                }
                match (last != size, &sender) {
                    (true, Some(s)) => {
                        last = size;
                        s.send(BufferSizeMessage(size)).await
                    },
                    _ => Ok(()),
                }?;
                if m_in.is_empty() && m_in.is_closed() && m_out.is_empty() && m_out.is_closed() {
                    return Ok(size as usize);
                }
                match buffer_movement {
                    false => {
                        back_off.wait().await;
                    },
                    true => {
                        back_off.reset();
                    },
                };
            }
        }

        let r_out_prep = r_a.join(r_b).join(
            total_in_buffer(std::mem::take(&mut self.buffer_size_monitor), monitor_i_rcv, monitor_o_rcv)
        ).await;

        fn structure_motion_result(input: ((MotionResult<usize>, MotionResult<usize>), Result<usize, SendError<BufferSizeMessage>>)) -> MotionResult<usize> {
            match input {
                ((MotionResult::Ok(stdin_count), MotionResult::Ok(_)), _x) => Ok(stdin_count),
                _ => Err(MotionError::NoneError),
            }
        }

        match structure_motion_result(r_out_prep) {
            Ok(x) => Ok(x),
            Err(x) => Err(x)
        }

    }
}


#[test]
fn do_stuff() {

    use crate::motion::IOData;
    use crate::connectable::Breakable;

    pub async fn test_buffer_impl() -> MotionResult<usize>  {
        use std::collections::VecDeque;

        async fn read_data(mut output: Pull) -> Vec<IOData> {
            let mut v: Vec<IOData> = vec![];
            async_std::task::sleep(std::time::Duration::from_millis(100)).await;
            loop {
                let x: MotionResult<crate::motion::IODataWrapper> = crate::motion::motion_read(&mut output, false).await;
                match x {
                    Ok(crate::motion::IODataWrapper::Finished) => {
                        return v;
                    }
                    Ok(crate::motion::IODataWrapper::IOData(x)) => {
                        v.push(x)
                    }
                    _ => {
                        return vec![];
                    }
                }
            }
        }

        async fn read_monitoring<X>(output: Receiver<X>) -> Vec<X> {
            let mut v: Vec<X> = vec![];
            loop {
                match output.recv().await {
                    Ok(x) => {
                        v.push(x);
                    },
                    Err(async_std::channel::RecvError) => {
                        return v;
                    }
                }
            }
        }

        fn get_input() -> VecDeque<IOData> {
            let mut vdq: VecDeque<IOData> = VecDeque::new();
            vdq.push_front(IOData(vec![68; 255]));
            vdq.push_front(IOData(vec![67; 255]));
            vdq.push_front(IOData(vec![66; 255]));
            vdq.push_front(IOData(vec![65; 255]));
            vdq
        }

        let input = Pull::Mock(PullJourney { src: 0, dst: 0 }, get_input());
        let mut buffer = Buffer::new();
        buffer.set_stdout_size(1);
        buffer.add_input(input, 0).unwrap();
        let output = buffer.add_output(OutputPort::Out, Breakable::Error, 0, 0).unwrap();
        let monitoring = buffer.add_buffer_size_monitor();
        let buffer_motion = buffer.start();
        match buffer_motion.join(read_data(output)).join(read_monitoring(monitoring)).await {
            ((Ok(proc_count), v), monitoring_msg) => {
                assert_eq!(
                    vec![
                        IOData(vec![65; 255]),
                        IOData(vec![66; 255]),
                        IOData(vec![67; 255]),
                        IOData(vec![68; 255]),
                    ],
                    v
                );

                assert_eq!(monitoring_msg.last(), Some(&BufferSizeMessage(0)));

                Ok(proc_count)
            },
            _ => {
                panic!("should have succeeded");
            }
        }
    }

    use async_std::task;
    println!("R: {:?}", task::block_on(test_buffer_impl()));
}

// struct BufferReturn {
//     stdout: Push, // Pull::IoReceiver || Pull::None
//     stderr: Push, // Pull::IoReceiver || Pull::None
//     future: Future<Output = ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>)>,
// }
// 
// 
// impl <E: IntoIterator<Item = (K, V)>,
//           A: IntoIterator<Item = R>,
//           R: AsRef<OsStr>,
//           O: AsRef<OsStr>,
//           K: AsRef<OsStr>,
//           V: AsRef<OsStr>,
//           P: AsRef<Path>> Buffer {
//     fn new(
//         stdin: Option<PullConfiguration>,
//         launch_spec: Buffer<E, P, O, A, K, V, R>,
//     ) -> Buffer {
//         Buffer {
//             stdout: None,
//             stderr: None,
//             stdin,
//             launch_spec,
//         }
//     }
// }
// 

// async fn get_command<E: IntoIterator<Item = (K, V)>,
//           A: IntoIterator<Item = R>,
//           R: AsRef<OsStr>,
//           O: AsRef<OsStr>,
//           K: AsRef<OsStr>,
//           V: AsRef<OsStr>,
//           P: AsRef<Path>>(stdin: Option<PullConfiguration>, launch_spec: Buffer<E, P, O, A, K, V, R>, outputs: BufferOutputs, monitoring: Sender<MonitorMessage>) -> BufferReturn
// {
// 
//     let outputs: (bool, bool) = match outputs {
//         BufferOutputs::STDOUT => (true, false),
//         BufferOutputs::STDOUT_AND_STDERR => (true, true),
//         BufferOutputs::STDERR => (false, true),
//     };
// 
//     let current_path: &Path = std::env::current_dir().expect("Unable to identify current $PATH").as_path();
//     let cmd = &launch_spec.command;
// 
//     let mut child_builder = aip::Command::new(cmd);
// 
//     child_builder.stdin(if stdin.is_some() { std::process::Stdio::piped() } else { std::process::Stdio::null() } );
//     child_builder.stderr(if outputs.1 { std::process::Stdio::piped() } else { std::process::Stdio::null() });
//     child_builder.stdout(if outputs.0 { std::process::Stdio::piped() } else { std::process::Stdio::null() });
// 
//     match launch_spec.path {
//         Some(p) => { child_builder.current_dir(p); },
//         None => ()
//     };
// 
//     match launch_spec.env {
//         Some(env) => { child_builder.envs(env.into_iter()); }
//         None => { child_builder.envs(std::env::vars_os()); }
//     }
// 
//     match launch_spec.args {
//         Some(args) => { child_builder.args(args); },
//         None => ()
//     };
// 
//     let child = child_builder.spawn().unwrap();
// 
// 
//     let mut child_stdin_pull = [match stdin {
//         Some(stdin) => { stdin },
//         None => PullConfiguration { priority: 0, id: 0, pull: Pull::None }
//     }];
// 
//     let mut child_stdin_push = [match child.stdin {
//         Some(stdin) => Push::CmdStdin(stdin),
//         None => Push::None,
//     }];
// 
//     // let mut io_sender = [];
//     let r1 = motion(&mut child_stdin_pull, monitoring.clone(), &mut child_stdin_push);
// 
//     let mut child_stdout_pull = [match child.stdout {
//         Some(stdout) => PullConfiguration { priority: 2, id: 2, pull: Pull::CmdStdout(stdout) },
//         None => PullConfiguration { priority: 2, id: 2, pull: Pull::None },
//     }];
// 
//     let mut child_stderr_pull = [match child.stderr {
//         Some(stderr) => PullConfiguration { priority: 2, id: 2, pull: Pull::CmdStderr(stderr) },
//         None => PullConfiguration { priority: 2, id: 2, pull: Pull::None },
//     }];
// 
//     let (child_stdout_push_channel, stdout_io_reciever_channel) = bounded(1);
//     let (child_stderr_push_channel, stderr_io_reciever_channel) = bounded(1);
// 
//     let mut child_stdout_push = [Push::IoSender(child_stdout_push_channel)];
//     let mut child_stderr_push = [Push::IoSender(child_stderr_push_channel)];
// 
//     let r2 = motion(&mut child_stdout_pull, monitoring.clone(), &mut child_stdout_push);
//     let r3 = motion(&mut child_stderr_pull, monitoring.clone(), &mut child_stderr_push);
// 
//     fn structure_motion_result(input: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>)) -> MotionResult<usize> {
//         match input {
//             ((MotionResult::Ok(stdin_count), MotionResult::Ok(_)), MotionResult::Ok(_)) => Ok(stdin_count),
//             _ => Err(MotionError::NoneError),
//         }
//     }
//     // let f = structure_motion_result(r1.join(r2).join(r3).await);
//     let f: Future<Output = ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>)> = r1.join(r2).join(r3);
// 
//     // BufferReturn {
//     //     stdout: Push::IoReceiver(stdout_io_reciever_channel),
//     //     stderr: Push::IoReceiver(stderr_io_reciever_channel),
//     //     future: f,
//     // }
//     // struct CommandStats {
//     // }
//     // let mut cmd_stdin = Push::CmdStdin(cmd.stdin.unwrap());
//     // let mut cmd_stdin = Pull::CmdStderr(child.stderr.unwrap());
//     // let mut cmd_stdout = Pull::CmdStdout(child.stdout.unwrap());
//     f
// }
