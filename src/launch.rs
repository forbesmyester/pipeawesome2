// use pipeawesome2::motion::{Pull, MotionResult, IOData};
use crate::motion::PullJourney;
use std::time::Instant;
use crate::motion::Journey;
use crate::connectable::ConnectableAddInputError;
use crate::connectable::ConnectableAddOutputError;
use crate::connectable::OutputPort;
use crate::connectable::Connectable;
use crate::connectable::Breakable;
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
    id: usize,
    command: O,
    path: Option<P>,
    env: Option<E>,
    args: Option<A>,
    stdout: Option<Push>,
    stderr: Option<Push>,
    exit_status: Option<(Journey, Sender<IOData>)>,
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

    fn add_output(&mut self, port: OutputPort, breakable: Breakable, src_id: usize, dst_id: usize) -> Result<Pull, ConnectableAddOutputError> {
        match port {
            OutputPort::Err => {
                if self.stderr.is_some() {
                    return Err(ConnectableAddOutputError::AlreadyAllocated(OutputPort::Err));
                }
                let (child_stdout_push_channel, chan_rx) = bounded(1);
                self.stderr = Some(Push::IoSender(Journey { src: src_id, dst: dst_id, breakable }, child_stdout_push_channel));
                Ok(Pull::Receiver(PullJourney { src: src_id, dst: dst_id }, chan_rx))
            },
            OutputPort::Exit => {
                if self.exit_status.is_some() {
                    return Err(ConnectableAddOutputError::AlreadyAllocated(OutputPort::Exit));
                }
                let (child_exit_status_push_channel, chan_rx) = bounded(1);
                self.exit_status = Some((Journey { src: src_id, dst: dst_id, breakable }, child_exit_status_push_channel));
                Ok(Pull::Receiver(PullJourney { src: src_id, dst: dst_id }, chan_rx))
            },
            OutputPort::Out => {
                if self.stdout.is_some() {
                    return Err(ConnectableAddOutputError::AlreadyAllocated(OutputPort::Out));
                }
                let (child_stdout_push_channel, chan_rx) = bounded(1);
                self.stdout = Some(Push::IoSender(Journey { src: src_id, dst: dst_id, breakable }, child_stdout_push_channel));
                Ok(Pull::Receiver(PullJourney { src: src_id, dst: dst_id }, chan_rx))
            }
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
        id: usize,
        env: Option<E>,
        path: Option<P>,
        command: O,
        args: Option<A>
    ) -> Launch<E, P, O, A, K, V, R> {
        Launch {
            id,
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


    pub async fn start(&mut self) -> MotionResult<usize> {

        assert!(!self.launched);

        let mut child_builder = aip::Command::new(&self.command);

        self.environment_configure(&mut child_builder);

        let (child_stdin_pull, child_stdin) = match std::mem::take(&mut self.stdin) {
            Some(stdin) => {
                ( stdin, std::process::Stdio::piped() )
            },
            None => {
                (
                    Pull::None,
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

        let mut child = child_builder.spawn().map_err(
            |_c| {
                MotionError::LaunchSpawnError(self.command.as_ref().to_str().map(|s| s.to_owned()))
            }
        ).unwrap();

        let child_stdin_push = match std::mem::take(&mut child.stdin) {
            Some(stdin) => {
                let src = child_stdin_pull.journey().map(|j| j.src).unwrap_or(self.id);
                Push::CmdStdin(Journey { src, dst: self.id, breakable: Breakable::Finish }, stdin)
            },
            None => Push::None,
        };

        let r_input = motion(child_stdin_pull, MotionNotifications::empty(), child_stdin_push);

        let (stdout_pull, stdout_push) = match (std::mem::take(&mut child.stdout), std::mem::take(&mut self.stdout)) {
            (Some(stdout), Some(push)) => {
                let dst = push.journey().map(|j| j.dst).unwrap_or(self.id);
                let pull = Pull::CmdStdout(PullJourney { src: self.id, dst }, stdout, ReadSplitControl::new());
                (pull, push)
            },
            _ => {
                (
                    Pull::None,
                    Push::None
                )
            }
        };

        let r2 = motion(stdout_pull, MotionNotifications::empty(), stdout_push);

        let (stderr_pull, stderr_push) = match (std::mem::take(&mut child.stderr), std::mem::take( &mut self.stderr)) {
            (Some(stderr), Some(push)) => {
                let dst = push.journey().map(|j| j.dst).unwrap_or(self.id);
                let pull = Pull::CmdStderr(PullJourney { src: self.id, dst }, stderr, ReadSplitControl::new());
                (pull, push)
            },
            _ => {
                (
                    Pull::None,
                    Push::None
                )
            }
        };

        let r3 = motion(stderr_pull, MotionNotifications::empty(), stderr_push);

        let r_out_prep: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>) = r_input.join(r2).join(r3).await;

        if let Some((journey, exit_status_tx)) = &mut self.exit_status {
            loop {
                let msg = match child.status().await.ok().map(|es| es.code()) {
                    Some(Some(exit_status)) => {
                        let str = format!("{:?}", exit_status);
                        let bytes = str.as_bytes();
                        Some(IOData(bytes.split_at(bytes.len()).0.iter().copied().collect()))
                    },
                    Some(None) => {
                        Some(IOData("_".as_bytes().to_vec())) // If killed, then no exit status available without using std::os::unix
                    }
                    None => None
                };
                match msg {
                    Some(m) => {
                        exit_status_tx.send(m).await.map_err(|e| MotionError::SendError(*journey, Instant::now(), e))?;
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
                ((MotionResult::Err(e), _), _) => Err(e),
                ((_, MotionResult::Err(e)), _) => Err(e),
                ((_, _), MotionResult::Err(e)) => Err(e),
            }
        }

        structure_motion_result(r_out_prep)

    }

}



