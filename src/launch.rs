// use pipeawesome2::motion::{Pull, MotionResult, IOData};
use crate::connectable::ConnectableAddInputError;
use crate::connectable::ConnectableAddOutputError;
use crate::connectable::OutputPort;
use crate::connectable::Connectable;
use std::ffi::OsStr;
use async_std::{channel::Sender, prelude::*};
use std::path::Path;
use crate::motion::{IOData, MotionError, MotionNotifications, ReadSplitControl};

use async_std::process as aip;
use async_std::channel::bounded;
use super::motion::{ motion, MotionResult, Pull, Push, };


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
    stdout: Option<Push>,
    stderr: Option<Push>,
    exit_status: Option<Sender<IOData>>,
    stdin: Option<Pull>,
    launched: bool,
}


impl <E: IntoIterator<Item = (K, V)>,
          P: AsRef<Path>,
          O: AsRef<OsStr>,
          A: IntoIterator<Item = R>,
          K: AsRef<OsStr>,
          V: AsRef<OsStr>,
          R: AsRef<OsStr>,
          > Connectable for Launch<E, P, O, A, K, V, R> {

    fn add_output(&mut self, port: OutputPort) -> Result<Pull, ConnectableAddOutputError> {
        match port {
            OutputPort::Err => {
                if self.stderr.is_some() {
                    return Err(ConnectableAddOutputError::AlreadyAllocated(OutputPort::Err));
                }
                let (child_stdout_push_channel, chan_rx) = bounded(1);
                self.stderr = Some(Push::IoSender(child_stdout_push_channel));
                Ok(Pull::Receiver(chan_rx))
            },
            OutputPort::Exit => {
                if self.exit_status.is_some() {
                    return Err(ConnectableAddOutputError::AlreadyAllocated(OutputPort::Exit));
                }
                let (child_exit_status_push_channel, chan_rx) = bounded(1);
                self.exit_status = Some(child_exit_status_push_channel);
                Ok(Pull::Receiver(chan_rx))
            },
            OutputPort::Out => {
                if self.stdout.is_some() {
                    return Err(ConnectableAddOutputError::AlreadyAllocated(OutputPort::Out));
                }
                let (child_stdout_push_channel, chan_rx) = bounded(1);
                self.stdout = Some(Push::IoSender(child_stdout_push_channel));
                Ok(Pull::Receiver(chan_rx))
            },
            x => Err(ConnectableAddOutputError::UnsupportedPort(x))
        }
    }


    fn add_input(&mut self, pull: Pull, unused_priority: isize) -> Result<(), ConnectableAddInputError> {
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
            exit_status: None,
            command,
            launched: false,
            path,
            env,
            args
        }
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

        if let Some(args) = std::mem::take(&mut self.args) {
            child_builder.args(args);
        }
    }


    async fn start_stream(&mut self) -> MotionResult<usize> {

        assert!(!self.launched);

        let mut child_builder = aip::Command::new(&self.command);

        self.environment_configure(&mut child_builder);

        let (child_stdin_pull, child_stdin) = match std::mem::take(&mut self.stdin) {
            Some(stdin) => {
                ( vec![stdin], std::process::Stdio::piped() )
            },
            None => {
                (
                    vec![Pull::None],
                    std::process::Stdio::null()
                )
            }
        };
        child_builder.stdin(child_stdin);

        match self.stdout.is_some() {
            true => { child_builder.stdout(async_std::process::Stdio::piped()); }
            false => { child_builder.stdout(async_std::process::Stdio::null()); }
        }

        match self.stderr.is_some() {
            true => { child_builder.stderr(async_std::process::Stdio::piped()); }
            false => { child_builder.stderr(async_std::process::Stdio::null()); }
        }

        let mut child = child_builder.spawn().unwrap();

        let child_stdin_push = vec![match std::mem::take(&mut child.stdin) {
            Some(stdin) => Push::CmdStdin(stdin),
            None => Push::None,
        }];

        let r1 = motion(child_stdin_pull, MotionNotifications::empty(), child_stdin_push);

        let (stdout_pull, stdout_push) = match (std::mem::take(&mut child.stdout), std::mem::take(&mut self.stdout)) {
            (Some(stdout), Some(push)) => {
                let pull = Pull::CmdStdout(stdout, ReadSplitControl::new());
                (vec![pull], vec![push])
            },
            _ => {
                (
                    vec![Pull::None],
                    vec![Push::None]
                )
            }
        };

        let r2 = motion(stdout_pull, MotionNotifications::empty(), stdout_push);

        let (stderr_pull, stderr_push) = match (std::mem::take(&mut child.stderr), std::mem::take( &mut self.stderr)) {
            (Some(stderr), Some(push)) => {
                let pull = Pull::CmdStderr(stderr, ReadSplitControl::new());
                (vec![pull], vec![push])
            },
            _ => {
                (
                    vec![Pull::None],
                    vec![Push::None]
                )
            }
        };

        let r3 = motion(stderr_pull, MotionNotifications::empty(), stderr_push);

        let r_out_prep: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>) = r1.join(r2).join(r3).await;
        if let Some(exit_status_tx) = &mut self.exit_status {
            loop {
                let msg = match child.try_status().ok().flatten().map(|es| es.code()) {
                    Some(Some(exit_status)) => {
                        let str = format!("{:?}", exit_status);
                        let bytes = str.as_bytes();
                        Some(IOData(bytes.split_at(bytes.len()).0.iter().copied().collect()))
                    },
                    Some(None) => {
                        Some(IOData(vec![]))
                    }
                    None => None
                };
                match msg {
                    Some(m) => {
                        exit_status_tx.send(m).await?;
                        exit_status_tx.close();
                        break;
                    },
                    None => { async_std::task::sleep(std::time::Duration::from_millis(100)).await; }
                }
            }
        }

        fn structure_motion_result(input: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>)) -> MotionResult<usize> {
            match input {
                ((MotionResult::Ok(stdin_count), MotionResult::Ok(_)), MotionResult::Ok(_)) => Ok(stdin_count),
                _ => Err(MotionError::NoneError),
            }
        }

        structure_motion_result(r_out_prep)

    }


    pub async fn start(&mut self) -> MotionResult<usize> {
        self.start_stream().await
    }


    // TODO: pub async fn start_per_line(&mut self) -> MotionResult<usize>
}



