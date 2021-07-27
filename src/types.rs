use async_std::channel::{Receiver, Sender};

pub type InputId = usize;
pub type InputIdGetter = Receiver<InputId>;
pub type InputIdSetter = Sender<InputId>;

