use crate::motion::PullJourney;
use crate::motion::Journey;
use crate::connectable::Breakable;
use std::time::Instant;
use crate::connectable::Connectable;
use crate::connectable::ConnectableAddInputError;
use crate::connectable::OutputPort;
use crate::connectable::ConnectableAddOutputError;
use super::motion::{ MotionError, MotionResult, MotionNotifications, Pull, Push, motion };

use crate::{startable_control::StartableControl};
use async_trait::async_trait;

#[allow(clippy::new_without_default)]
pub struct Drain {
    started: bool,
    stdin: Option<Pull>,
    write_location: Option<String>,
}

impl Drain {

    pub fn new(write_location: String) -> Drain {
        Drain {
            started: false,
            stdin: None,
            write_location: Some(write_location)
        }
    }

}


impl Connectable for Drain {

    fn add_output(&mut self, port: OutputPort, _breakable: Breakable, _src_id: usize, _dst_id: usize) -> std::result::Result<Pull, ConnectableAddOutputError> {
        Err(ConnectableAddOutputError::UnsupportedPort(port))
    }

    fn add_input(&mut self, pull: Pull, unused_priority: isize) -> std::result::Result<(), ConnectableAddInputError> {
        if unused_priority != 0 {
            return Err(ConnectableAddInputError::UnsupportedPriority(unused_priority));
        }
        if !self.stdin.is_none() {
            return Err(ConnectableAddInputError::AlreadyAllocated);
        }
        self.stdin = Some(pull);
        Ok(())
    }

}



#[async_trait]
impl StartableControl for Drain {
    async fn start(&mut self) -> MotionResult<usize> {


        self.started = true;
        if let Some(pull) = std::mem::take(&mut self.stdin) {

            let push = match (pull.journey(), std::mem::take(&mut self.write_location)) {
                (Some(PullJourney { src, dst }), Some(f)) if f == "-" => Push::Stdout(Journey { src: *src, dst: *dst, breakable: Breakable::Terminate }, async_std::io::stdout()),
                (Some(PullJourney { src, dst }), Some(f)) if f == "_" => Push::Stderr(Journey { src: *src, dst: *dst, breakable: Breakable::Terminate }, async_std::io::stderr()),
                (Some(PullJourney { src, dst }), Some(filename)) => {
                    let breakable = Breakable::Terminate;
                    let file = async_std::fs::File::create(filename).await
                        .map_err(|e| MotionError::OpenIOError(PullJourney { src: *src, dst: *dst }, Instant::now(), e))?;
                    Push::File(Journey { src: *src, dst: *dst, breakable }, async_std::io::BufWriter::new(file))
                },
                _ => Push::None,
            };

            return motion(pull, MotionNotifications::empty(), push).await
        }
        MotionResult::Err(MotionError::NoneError)
    }
}



