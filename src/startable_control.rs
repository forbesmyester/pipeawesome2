use async_trait::async_trait;
use super::motion::MotionResult;

#[async_trait]
pub trait StartableControl {
    async fn start(&mut self) -> MotionResult<usize>;
}



