// use pipeawesome2::motion::{Pull, MotionResult, IOData};
use std::ffi::OsStr;
use async_std::prelude::*;
use std::path::Path;
use crate::motion::{MotionError, MotionNotifications};

use async_std::process as aip;
use async_std::channel::bounded;
use super::motion::{ motion, MotionResult, PullConfiguration, Pull, Push, };

// #[derive(Debug)]
// enum LaunchOutputs {
//     STDOUT,
//     STDERR,
//     STDOUT_AND_STDERR,
// }

pub struct Launch<E, P, O, A, K, V, R>
    where E: IntoIterator<Item = (K, V)>,
          P: AsRef<Path>,
          O: AsRef<OsStr>,
          A: IntoIterator<Item = R>,
          K: AsRef<OsStr>,
          V: AsRef<OsStr>,
          R: AsRef<OsStr>
{
    command: O,
    path: Option<P>,
    env: Option<E>,
    args: Option<A>,
    stdout: Option<Push>, // Pull::IoReceiver || Pull::None
    stderr: Option<Push>, // Pull::IoReceiver || Pull::None
    stdin: Option<PullConfiguration>,
    launched: bool,
}

impl <E: IntoIterator<Item = (K, V)>,
          P: AsRef<Path>,
          O: AsRef<OsStr>,
          A: IntoIterator<Item = R>,
          K: AsRef<OsStr>,
          V: AsRef<OsStr>,
          R: AsRef<OsStr>,
          > Launch<E, P, O, A, K, V, R> {
    pub fn new(
        env: Option<E>,
        path: Option<P>,
        command: O,
        args: Option<A>
    ) -> Launch<E, P, O, A, K, V, R> {
        Launch {
            stdin: None,
            stdout: None,
            stderr: None,
            command,
            launched: false,
            path,
            env,
            args
        }
    }

    pub fn add_stdin(&mut self, pull: Pull, priority: u8) {
        self.stdin = Some(PullConfiguration { priority, pull, id: 0 });
    }

    pub fn add_stdout(&mut self) -> Pull {
        assert!(self.stdout.is_none());
        let (child_stdout_push_channel, stdout_io_reciever_channel) = bounded(1);
        self.stdout = Some(Push::IoSender(child_stdout_push_channel));
        Pull::IoReceiver(stdout_io_reciever_channel)
    }

    pub fn add_stderr(&mut self) -> Pull {
        assert!(self.stdout.is_none());
        let (child_stdout_push_channel, stdout_io_reciever_channel) = bounded(1);
        self.stderr = Some(Push::IoSender(child_stdout_push_channel));
        Pull::IoReceiver(stdout_io_reciever_channel)
    }

    fn environment_configure(&mut self, child_builder: &mut aip::Command) {
        let current_path = std::env::current_dir();
        match (&self.path, current_path) {
            (Some(p), _) => { child_builder.current_dir(p); },
            (None, Ok(p)) => { child_builder.current_dir(p); },
            _ => (),
        };

        match &self.path {
            Some(p) => { child_builder.current_dir(p); },
            None => ()
        };

        match std::mem::take(&mut self.env) {
            Some(env) => { child_builder.envs(env.into_iter()); }
            None => { child_builder.envs(std::env::vars_os()); }
        }

        match std::mem::take(&mut self.args) {
            Some(args) => { child_builder.args(args); },
            None => ()
        };
    }

    pub async fn start(&mut self) -> MotionResult<usize> {

        assert!(self.launched == false);

        let mut child_builder = aip::Command::new(&self.command);

        self.environment_configure(&mut child_builder);

        let (child_stdin_pull, child_stdin) = match std::mem::take(&mut self.stdin) {
            Some(stdin) => {
                ( vec![stdin], std::process::Stdio::piped() )
            },
            None => {
                (
                    vec![PullConfiguration { priority: 0, id: 0, pull: Pull::None }],
                    std::process::Stdio::null()
                )
            }
        };
        child_builder.stdin(child_stdin);
        let child = child_builder.spawn().unwrap();

        let child_stdin_push = vec![match child.stdin {
            Some(stdin) => Push::CmdStdin(stdin),
            None => Push::None,
        }];

        // let mut io_sender = [];
        let r1 = motion(child_stdin_pull, MotionNotifications::empty(), child_stdin_push);

        let (stdout_pull, stdout_push) = match (child.stdout, std::mem::take(&mut self.stdout)) {
            (Some(stdout), Some(push)) => {
                let pull = PullConfiguration { priority: 2, id: 6, pull: Pull::CmdStdout(stdout) };
                (vec![pull], vec![push])
            },
            _ => (
                    vec![PullConfiguration { priority: 2, id: 7, pull: Pull::None }],
                    vec![Push::None]
                ),
        };

        let r2 = motion(stdout_pull, MotionNotifications::empty(), stdout_push);

        let (stderr_pull, stderr_push) = match (child.stderr, std::mem::take( &mut self.stderr)) {
            (Some(stderr), Some(push)) => {
                let pull = PullConfiguration { priority: 2, id: 8, pull: Pull::CmdStderr(stderr) };
                (vec![pull], vec![push])
            },
            _ => (
                    vec![PullConfiguration { priority: 2, id: 9, pull: Pull::None }],
                    vec![Push::None]
                ),
        };

        let r3 = motion(stderr_pull, MotionNotifications::empty(), stderr_push);

        let r_out_prep: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>) = r1.join(r2).join(r3).await;

        fn structure_motion_result(input: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>)) -> MotionResult<usize> {
            match input {
                ((MotionResult::Ok(stdin_count), MotionResult::Ok(_)), MotionResult::Ok(_)) => Ok(stdin_count),
                _ => Err(MotionError::NoneError),
            }
        }

        structure_motion_result(r_out_prep)

    }
}


// struct LaunchReturn {
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
//           P: AsRef<Path>> Launch {
//     fn new(
//         stdin: Option<PullConfiguration>,
//         launch_spec: Launch<E, P, O, A, K, V, R>,
//     ) -> Launch {
//         Launch {
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
//           P: AsRef<Path>>(stdin: Option<PullConfiguration>, launch_spec: Launch<E, P, O, A, K, V, R>, outputs: LaunchOutputs, monitoring: Sender<MonitorMessage>) -> LaunchReturn
// {
// 
//     let outputs: (bool, bool) = match outputs {
//         LaunchOutputs::STDOUT => (true, false),
//         LaunchOutputs::STDOUT_AND_STDERR => (true, true),
//         LaunchOutputs::STDERR => (false, true),
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
//     // LaunchReturn {
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
