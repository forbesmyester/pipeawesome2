// use pipeawesome2::motion::{Pull, MotionResult, IOData};
use async_std::{channel::SendError, prelude::*};

use async_std::channel::{bounded, unbounded, Receiver, Sender };
use crate::motion::{IOData, MotionNotifications};

use super::monitor::MonitorMessage;
use super::motion::{ motion, MotionError, MotionResult, PullConfiguration, Pull, Push, };

pub struct Buffer {
    stdout_size: usize,
    stdout: Option<Push>,
    stdin: Option<Pull>,
    buffer_size_monitor: Option<Sender<usize>>,
}

/**
 * Buffer
 *
 * STDIN -> UnboundedChannelSender -> UnboundedChannelReciever -> STDOUT
 */
impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            stdout_size: 8,
            stdin: None,
            stdout: None,
            buffer_size_monitor: None,
        }
    }

    pub fn add_buffer_size_monitor(&mut self) -> Receiver<usize> {
        let (tx, rx) = bounded(self.stdout_size);
        self.buffer_size_monitor = Some(tx);
        rx
    }

    pub fn set_stdout_size(&mut self, size: usize) {
        self.stdout_size = size;
    }

    pub fn add_stdin(&mut self, pull: Pull, _priority: u8) {
        assert!(self.stdin.is_none());
        self.stdin = Some(pull);
    }

    pub fn add_stdout(&mut self) -> Pull {
        assert!(self.stdout.is_none());
        let (child_stdout_push_channel, stdout_io_reciever_channel) = bounded(self.stdout_size);
        self.stdout = Some(Push::IoSender(child_stdout_push_channel));
        Pull::IoReceiver(stdout_io_reciever_channel)
    }

    pub async fn start(&mut self) -> MotionResult<usize> {

        let (unbounded_snd, unbounded_rcv) = unbounded();
        let (monitor_i_snd, monitor_i_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
        let (monitor_o_snd, monitor_o_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);

        let pull_a = vec![PullConfiguration { priority: 0, id: 8, pull: std::mem::take(&mut self.stdin).unwrap() }];
        let push_a = vec![Push::Sender(unbounded_snd)];

        let pull_b = vec![PullConfiguration { priority: 0, id: 9, pull: Pull::Receiver(unbounded_rcv) }];

        let r_a = motion(pull_a, MotionNotifications::written(monitor_i_snd), push_a);
        let r_b = motion(pull_b, MotionNotifications::read(monitor_o_snd), vec![std::mem::take(&mut self.stdout).unwrap()]);

        async fn total_in_buffer(sender: Option<Sender<usize>>, m_in: Receiver<MonitorMessage>, m_out: Receiver<MonitorMessage>) -> Result<usize, SendError<usize>> {
            let mut size: usize = 0;
            loop {
                let mut changed = false;
                match m_in.recv().await {
                    Err(_x) => (),
                    Ok(_) => {
                        println!("SIZEI: W");
                        changed = true;
                        size = size + 1;
                    }
                }
                match &sender {
                    Some(s) => { s.send(size).await?; },
                    None => ()
                };
                if size > 0 {
                    match m_out.recv().await {
                        Err(_x) => (),
                        Ok(_) => {
                            println!("SIZEO: R");
                            size = size - 1;
                            changed = true;
                        }
                    }
                }
                match (changed, &sender) {
                    (true, Some(s)) => s.send(size).await,
                    _ => Ok(()),
                }?;
                if m_in.is_closed() && m_out.is_closed() {
                    return Ok(size);
                }
            }
        }

        println!(">> {:?}", self.buffer_size_monitor);
        let r_out_prep = r_a.join(r_b).join(total_in_buffer(std::mem::take(&mut self.buffer_size_monitor), monitor_i_rcv, monitor_o_rcv)).await;

        fn structure_motion_result(input: ((MotionResult<usize>, MotionResult<usize>), Result<usize, SendError<usize>>)) -> MotionResult<usize> {
            match input {
                ((MotionResult::Ok(stdin_count), MotionResult::Ok(_)), _x) => Ok(stdin_count),
                _ => Err(MotionError::NoneError),
            }
        }

        structure_motion_result(r_out_prep)

    }
}

pub async fn test_buffer_impl() -> MotionResult<usize>  {
    use std::collections::VecDeque;

    async fn read_data(mut output: Pull) -> Vec<IOData> {
        let mut v: Vec<IOData> = vec![];
        loop {
            let x: MotionResult<IOData> = crate::motion::motion_read(&mut output).await;
            match x {
                Ok(IOData::Finished) => {
                    v.push(IOData::Finished);
                    return v;
                }
                Ok(x) => {
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
                    println!(">>> PUSH");
                    v.push(x);
                },
                Err(async_std::channel::RecvError) => {
                    println!(">>> RET");
                    return v;
                }
            }
        }
    }

    fn get_input() -> VecDeque<IOData> {
        let mut vdq: VecDeque<IOData> = VecDeque::new();
        let vdq_data_0: [u8; 255] = [65; 255];
        let vdq_data_1: [u8; 255] = [66; 255];
        vdq.push_front(IOData::Finished);
        vdq.push_front(IOData::Data(8, vdq_data_1));
        vdq.push_front(IOData::Data(8, vdq_data_0));
        vdq
    }

    let input = Pull::IoMock(get_input());
    let mut buffer = Buffer::new();
    buffer.set_stdout_size(1);
    buffer.add_stdin(input, 1);
    let output = buffer.add_stdout();
    let monitoring = buffer.add_buffer_size_monitor();
    let buffer_motion = buffer.start();
    match buffer_motion.join(read_data(output)).join(read_monitoring(monitoring)).await {
        ((Ok(proc_count), mut v), monitoring_msg) => {
            println!("MONITORING_MESSAGES: {:?}", monitoring_msg);
            assert_eq!(Some(IOData::Finished), v.pop());
            assert_eq!(Some(IOData::Data(8, [66; 255])), v.pop());
            assert_eq!(Some(IOData::Data(8, [65; 255])), v.pop());
            Ok(proc_count)
        },
        _ => {
            panic!("should have succeeded");
        }
    }
}

#[test]
fn do_stuff() {
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
