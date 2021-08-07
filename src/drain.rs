use super::motion::{ MotionResult, MotionNotifications, Pull, Push, motion };

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
        assert!(self.stdin.len() == 0);
        self.stdin.push(pull);
    }

    pub async fn start(&mut self) -> MotionResult<usize> {
        self.started = true;
        motion(std::mem::take(&mut self.stdin), MotionNotifications::empty(), std::mem::take(&mut self.stdout)).await
    }

}



