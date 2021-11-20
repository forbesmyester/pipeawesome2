use crate::connectable::Connectable;
use crate::connectable::ConnectableAddInputError;
use crate::connectable::OutputPort;
use crate::connectable::ConnectableAddOutputError;
use super::motion::{ MotionResult, MotionNotifications, Pull, Push, motion };

use crate::{startable_control::StartableControl};
use async_trait::async_trait;

#[allow(clippy::new_without_default)]
pub struct Drain {
    started: bool,
    stdin: Vec<Pull>,
    stdout: Vec<Push>,
}

impl Drain {

    pub fn new(push: Push) -> Drain {
        Drain {
            started: false,
            stdin: vec![],
            stdout: vec![push],
        }
    }

    pub fn add_stdin(&mut self, pull: Pull) {
        assert!(!self.started);
        assert!(self.stdin.is_empty());
        self.stdin.push(pull);
    }

}


impl Connectable for Drain {

    fn add_output(&mut self, _port: OutputPort) -> std::result::Result<Pull, ConnectableAddOutputError> {
        Err(ConnectableAddOutputError::UnsupportedForControl)
    }

    fn add_input(&mut self, pull: Pull, unused_priority: isize) -> std::result::Result<(), ConnectableAddInputError> {
        if unused_priority != 0 {
            return Err(ConnectableAddInputError::UnsupportedPriority(unused_priority));
        }
        if !self.stdin.is_empty() {
            return Err(ConnectableAddInputError::AlreadyAllocated);
        }
        self.stdin.push(pull);
        Ok(())
    }

}



#[async_trait]
impl StartableControl for Drain {
    async fn start(&mut self) -> MotionResult<usize> {
        self.started = true;
        motion(std::mem::take(&mut self.stdin), MotionNotifications::empty(), std::mem::take(&mut self.stdout)).await
    }

}



