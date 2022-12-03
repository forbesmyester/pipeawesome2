use std::str::FromStr;

use crate::motion::Pull;
use crate::config::ComponentType;
use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Breakable {
    Consume,
    Finish,
    Terminate
}

#[derive(Copy, Debug, Eq, Hash, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputPort {
    Err,
    Out,
    Exit,
}

#[derive(Debug)]
pub enum ParseOutputPortErr {
    UnknownStr(String)
}


impl FromStr for OutputPort {
    type Err = ParseOutputPortErr;

    fn from_str(s: &str) -> Result<Self, <OutputPort as FromStr>::Err> {
        match s {
            "err" => Ok(OutputPort::Err),
            "Err" => Ok(OutputPort::Err),
            "ERR" => Ok(OutputPort::Err),
            "exit" => Ok(OutputPort::Exit),
            "Exit" => Ok(OutputPort::Exit),
            "EXIT" => Ok(OutputPort::Exit),
            "out" => Ok(OutputPort::Out),
            "Out" => Ok(OutputPort::Out),
            "OUT" => Ok(OutputPort::Out),
            _ => Err(ParseOutputPortErr::UnknownStr(s.to_string())),
        }
    }
}


fn default_input_port_breakable() -> Breakable { Breakable::Terminate }

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct InputPort {
    #[serde(default = "default_input_port_breakable")]
    pub breakable: Breakable,
    pub priority: isize,
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
    fn add_output(&mut self, port: OutputPort, breakable: Breakable, src_id: usize, dst_id: usize) -> Result<Pull, ConnectableAddOutputError>;
    fn add_input(&mut self, pull: Pull, priority: isize) -> Result<(), ConnectableAddInputError>;
}

