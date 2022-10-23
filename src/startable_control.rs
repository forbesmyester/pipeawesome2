use async_std::channel::Sender;
use async_trait::async_trait;
use crate::motion::SpyMessage;

use super::motion::MotionResult;

#[async_trait]
pub trait StartableControl {
    async fn start(&mut self, mut spy: Option<Sender<SpyMessage>>) -> MotionResult<usize>;
}



