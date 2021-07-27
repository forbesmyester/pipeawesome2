// use pipeawesome2::motion::{Pull, MotionResult, IOData};
use async_std::{channel::Receiver, prelude::*, task};

use async_std::channel::{bounded, unbounded, Sender };
use crate::motion::IOData;

use super::motion::{ motion, MotionError, MotionResult, MotionNotifications, PullConfiguration, Pull, Push, };

pub struct Junction {
    stdout_size: usize,
    started: bool,
    stdout: Vec<Push>,
    stdin: Vec<PullConfiguration>,
}

/**
 * Buffer
 *
 * STDIN -> UnboundedChannelSender -> UnboundedChannelReciever -> STDOUT
 */
impl Junction {
    pub fn new() -> Junction {
        Junction {
            stdout_size: 8,
            started: false,
            stdin: vec![],
            stdout: vec![],
        }
    }

    pub fn set_stdout_size(&mut self, size: usize) {
        self.stdout_size = size;
    }

    pub fn add_stdin(&mut self, pull: Pull, priority: u8) {
        assert!(!self.started);
        let id = self.stdin.len() + self.stdout.len();
        self.stdin.push(PullConfiguration { priority, id, pull });
    }

    pub fn add_stdout(&mut self) -> Pull {
        assert!(!self.started);
        let (child_stdout_push_channel, stdout_io_reciever_channel) = bounded(self.stdout_size);
        self.stdout.push(Push::IoSender(child_stdout_push_channel));
        Pull::IoReceiver(stdout_io_reciever_channel)
    }

    pub async fn start(&mut self) -> MotionResult<usize> {
        motion(std::mem::take(&mut self.stdin), MotionNotifications::empty(), std::mem::take(&mut self.stdout)).await
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
    let mut junction = Junction::new();
    junction.set_stdout_size(1);
    junction.add_stdin(input, 1);
    let output = junction.add_stdout();
    let junction_motion = junction.start();
    match junction_motion.join(read_data(output)).await {
        (Ok(proc_count), mut v) => {
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

