use crate::{faucet::Faucet, junction::Junction, launch::Launch, buffer::Buffer, drain::Drain, motion::MotionResult };
use std::collections::HashMap;
use async_std::prelude::*;

pub struct Waiter {
    faucet: Vec<Faucet>,
    launch: Vec<Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String>>,
    junction: Vec<Junction>,
    buffer: Vec<Buffer>,
    drain: Vec<Drain>,
}

impl Waiter {

    pub fn new() -> Waiter {
        Waiter {
            faucet: vec![],
            launch: vec![],
            junction: vec![],
            buffer: vec![],
            drain: vec![],
        }
    }

    pub fn add_launch(&mut self, l: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String>) {
        self.launch.push(l);
    }

    pub fn add_faucet(&mut self, f: Faucet) {
        self.faucet.push(f);
    }

    pub fn add_junction(&mut self, j: Junction) {
        self.junction.push(j);
    }

    pub fn add_buffer(&mut self, b: Buffer) {
        self.buffer.push(b);
    }

    pub fn add_drain(&mut self, d: Drain) {
        self.drain.push(d);
    }

    pub async fn start(&mut self) -> MotionResult<usize> {
        use futures::future::join_all;

        fn folder(acc: MotionResult<usize>, cur: &mut MotionResult<usize>) -> MotionResult<usize> {
            match (acc, cur) {
                (MotionResult::Ok(i), MotionResult::Ok(j)) => MotionResult::Ok(i + *j),
                (Err(x), _) => Err(x),
                (_, x) => {
                    let mut y = MotionResult::Ok(0);
                    std::mem::swap(x, &mut y);
                    y
                }
            }
        }

        let faucets = join_all(self.faucet.iter_mut().map(|f| f.start()));
        let launch = join_all(self.launch.iter_mut().map(|l| l.start()));
        let junction = join_all(self.junction.iter_mut().map(|j| j.start()));
        let buffers = join_all(self.buffer.iter_mut().map(|b| b.start()));
        let drain = join_all(self.drain.iter_mut().map(|d| d.start()));

        let (mut f, (mut l, (mut j, (mut b, mut d)))) = faucets.join(launch.join(junction.join(buffers.join(drain)))).await;
        [
            f.iter_mut().fold(MotionResult::Ok(0), folder),
            l.iter_mut().fold(MotionResult::Ok(0), folder),
            j.iter_mut().fold(MotionResult::Ok(0), folder),
            b.iter_mut().fold(MotionResult::Ok(0), folder),
            d.iter_mut().fold(MotionResult::Ok(0), folder),
        ].iter_mut().fold(MotionResult::Ok(0), folder)
    }
}


