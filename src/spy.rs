use async_std::channel::{bounded, SendError};
use async_std::channel::{Sender, Receiver};

pub type SpyResult = Result<(), SpyMessage>;


pub struct SpySender {
    src: usize,
    sender: Sender<SpyMessage>,
}

impl SpySender {
    pub async fn send(&self, msg: Vec<u8>) -> Result<(), SendError<SpyMessage>> {
        self.sender.send(SpyMessage { src: self.src, msg }).await
    }
}

pub struct SpyMessage {
    src: usize,
    msg: Vec<u8>,
}

pub struct SpyFactory {
    src: usize,
    channels: Option<(Sender<SpyMessage>, Receiver<SpyMessage>)>,
}

impl SpyFactory {

    pub fn new(src: usize) -> SpyFactory {
        SpyFactory {
            src: src,
            channels: Some(bounded(8)),
        }
    }

    pub fn get_sender(&mut self) -> Option<SpySender> {
        match &mut self.channels {
            None => None,
            Some((sender, _)) => Some(SpySender { src: self.src, sender: sender.clone() }),
        }
    }

    pub fn get_receiver(&mut self) -> Option<Receiver<SpyMessage>> {
        if let Some((_, recvr)) = std::mem::take(&mut self.channels) {
            return Some(recvr)
        }
        None
    }

}
