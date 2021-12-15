use crate::connectable::Connectable;
use crate::connectable::ConnectableAddInputError;
use crate::connectable::OutputPort;
use crate::connectable::ConnectableAddOutputError;
use super::motion::{ MotionError, MotionResult, MotionNotifications, Pull, Push, motion };

use crate::{startable_control::StartableControl};
use async_trait::async_trait;

#[allow(clippy::new_without_default)]
pub struct Drain {
    id: usize,
    started: bool,
    stdin: Option<Pull>,
    stdout: Option<Push>,
}

impl Drain {

    pub fn new(id: usize, push: Push) -> Drain {
        Drain {
            id,
            started: false,
            stdin: None,
            stdout: Some(push),
        }
    }

}


impl Connectable for Drain {

    fn add_output(&mut self, port: OutputPort) -> std::result::Result<Pull, ConnectableAddOutputError> {
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
        if let (Some(pull), Some(push)) = (std::mem::take(&mut self.stdin), std::mem::take(&mut self.stdout)) {
            return motion(pull, MotionNotifications::empty(), push).await
        }
        println!("HERE");
        MotionResult::Err(MotionError::NoneError)
    }
}



