use crate::motion::Pull;
use crate::config::ComponentType;
use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputPort {
    Err,
    Out,
    Exit,
    // Size,
    Overflow,
}



#[derive(Debug)]
pub enum ConnectableAddOutputError {
    AlreadyAllocated(OutputPort),
    UnsupportedPort(OutputPort),
}

#[derive(Debug)]
pub enum ConnectableAddInputError {
    AlreadyAllocated,
    UnsupportedPriority(isize),
    UnsupportedForControl
}


#[derive(Debug)]
pub enum ConnectableErrorSource {
    Source(ComponentType, String),
}


#[derive(Debug)]
pub enum ConnectableError {
    AddInput(usize, ConnectableAddInputError),
    AddOutput(usize, ConnectableAddOutputError),
    CouldNotFindSourceComponent(usize),
    CouldNotFindDestinationComponent(usize),
}

impl std::fmt::Display for ConnectableError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Connectable {
    fn add_output(&mut self, port: OutputPort) -> Result<Pull, ConnectableAddOutputError>;
    fn add_input(&mut self, pull: Pull, priority: isize) -> Result<(), ConnectableAddInputError>;
}

