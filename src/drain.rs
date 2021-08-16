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

#[async_trait]
impl StartableControl for Drain {
    async fn start(&mut self) -> MotionResult<usize> {
        self.started = true;
        motion(std::mem::take(&mut self.stdin), MotionNotifications::empty(), std::mem::take(&mut self.stdout)).await
    }

}



