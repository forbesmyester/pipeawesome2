use crate::config::{Config, DeserializedConnection };
use crate::config::FaucetConfig;
use crate::motion::Push;
use crate::motion::Pull;
use crate::motion::ReadSplitControl;
use crate::config::InputPort;
use crate::config::ComponentType;
use std::collections::HashSet;
use crate::config::Connection;
use crate::connectable::ConnectableErrorSource;
use crate::connectable::ConnectableError;
use crate::connectable::{ Connectable, ConnectableAddOutputError, OutputPort };
use crate::{buffer::{Buffer, BufferSizeMessage}, drain::Drain, faucet::{Faucet, FaucetControl}, junction::Junction, launch::Launch, motion::MotionError, startable_control::StartableControl};
use std::collections::HashMap;
use async_std::{channel::Receiver, prelude::*};

type LaunchString = Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String>;


struct FaucetSettings {
    low_watermark: usize,
    high_watermark: usize,
    buffer_names: Vec<String>,
}


struct FaucetSettingsCapture {
    low_watermark: usize,
    high_watermark: usize,
    faucet_component: FaucetControl,
    buffer_size_monitors: Vec<Receiver<BufferSizeMessage>>,
}


pub struct JoinFrom<'a> {
    pub component_type: ComponentType,
    pub component_name: &'a str,
    pub output_port: OutputPort,
}


pub struct JoinTo<'a> {
    pub component_type: ComponentType,
    pub component_name: &'a str,
    pub input_priority: isize,
}


async fn manage_taps_from_buffers(mut faucet_settings: FaucetSettingsCapture) -> Result<usize, WaiterError> {

    let mut back_off = crate::back_off::BackOff::new();

    #[derive(Debug)]
    struct Bsm { pub buffer_size: usize, pub buffer_monitor: Receiver<BufferSizeMessage>, }

    let mut monitors: Vec<Bsm> = faucet_settings.buffer_size_monitors.into_iter().map(|bm| {
        Bsm {buffer_size: 0, buffer_monitor: bm }
    }).collect();

    #[derive(Debug, PartialEq)]
    enum FaucetStatus {
        Open,
        Closed,
    }

    let mut read_count = 0;
    let mut faucet_status = FaucetStatus::Open;
    loop {
        let mut to_remove = vec![];
        let mut nothing_read = true;
        let mut total = 0;
        for (index, buffer_size_monitor) in monitors.iter_mut().enumerate() {
            match buffer_size_monitor.buffer_monitor.try_recv() {
                Err(async_std::channel::TryRecvError::Closed) => {
                    to_remove.push(index);
                },
                Err(async_std::channel::TryRecvError::Empty) => {
                    total += buffer_size_monitor.buffer_size;
                },
                Ok(BufferSizeMessage(n)) => {
                    buffer_size_monitor.buffer_size = n;
                    total += n;
                    read_count += 1;
                    nothing_read = false;
                }
            }
        }
        if faucet_status == FaucetStatus::Open && total >= faucet_settings.high_watermark {
            faucet_settings.faucet_component.pause().await.map_err(|_| WaiterError::CouldNotPause)?;
            faucet_status = FaucetStatus::Closed;
        }
        if faucet_status == FaucetStatus::Closed && total <= faucet_settings.low_watermark {
            faucet_settings.faucet_component.resume().await.map_err(|_| WaiterError::CouldNotResume)?;
            faucet_status = FaucetStatus::Open;
        }
        //println!("TOTAL: {:?}: {:?}", faucet_status, total);
        for tr in to_remove.into_iter().rev() {
            monitors.remove(tr);
        }
        if monitors.is_empty() {
            return Ok(read_count);
        }
        match nothing_read {
            true => {
                back_off.wait().await
            },
            false => {
                back_off.reset()
            },
        };
    }
}


pub struct Waiter {
    faucet: HashMap<String, Faucet>,
    launch: HashMap<String, LaunchString>,
    junction: HashMap<String, Junction>,
    buffer: HashMap<String, Buffer>,
    drain: HashMap<String, Drain>,
    faucet_settings: HashMap<String, FaucetSettings>,
}


#[derive(Debug)]
pub enum WaiterError {
    CausedByError(String, MotionError),
    SettingsForMissingFaucet(String),
    SettingsRefersMissingBuffer(String),
    CouldNotPause,
    CouldNotResume,
}


#[allow(clippy::new_without_default)]
impl Waiter {

    pub fn new() -> Waiter {
        Waiter {
            faucet: HashMap::new(),
            launch: HashMap::new(),
            junction: HashMap::new(),
            buffer: HashMap::new(),
            drain: HashMap::new(),
            faucet_settings: HashMap::new(),
        }
    }


    fn get_src_pull(&mut self, src: JoinFrom) -> Result<Pull, ConnectableError> {

        let error_src = ConnectableErrorSource::Source(src.component_type.clone(), src.component_name.to_string());

        let r = match src {
            JoinFrom { component_type: ComponentType::Faucet, component_name, output_port } => {
                self.faucet.get_mut(component_name).map(|x| x.add_output(output_port))
            },
            JoinFrom { component_type: ComponentType::Launch, component_name, output_port } => {
                self.launch.get_mut(component_name).map(|x| x.add_output(output_port))
            },
            JoinFrom { component_type: ComponentType::Junction, component_name, output_port } => {
                self.junction.get_mut(component_name).map(|x| x.add_output(output_port))
            },
            JoinFrom { component_type: ComponentType::Buffer, component_name, output_port } => {
                self.buffer.get_mut(component_name).map(|x| x.add_output(output_port))
            },
            JoinFrom { component_type: ComponentType::Drain, component_name, output_port } => {
                self.drain.get_mut(component_name).map(|x| x.add_output(output_port))
            },
        };

        match r {
            None => Err(ConnectableError::CouldNotFindSourceComponent(error_src)),
            Some(Err(x)) => Err(ConnectableError::AddOutput(error_src, x)),
            Some(Ok(x)) => Ok(x),
        }
    }

    pub fn join(&mut self, src: JoinFrom, dst: JoinTo) -> Result<(), ConnectableError> {

        let error_dst = ConnectableErrorSource::Source(dst.component_type.clone(), dst.component_name.to_string());

        let pull = self.get_src_pull(src)?;

        let res = match dst {
            JoinTo { component_type: ComponentType::Faucet, component_name, input_priority } => {
                self.faucet.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
            JoinTo { component_type: ComponentType::Launch, component_name, input_priority } => {
                self.launch.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
            JoinTo { component_type: ComponentType::Junction, component_name, input_priority } => {
                self.junction.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
            JoinTo { component_type: ComponentType::Buffer, component_name, input_priority } => {
                self.buffer.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
            JoinTo { component_type: ComponentType::Drain, component_name, input_priority } => {
                self.drain.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
        };

        match res {
            None => Err(ConnectableError::CouldNotFindDestinationComponent(error_dst)),
            Some(Err(x)) => Err(ConnectableError::AddInput(error_dst, x)),
            Some(Ok(x)) => Ok(x),
        }

    }

    pub fn add_launch(&mut self, name: String, l: LaunchString) {
        self.launch.insert(name, l);
    }

    pub fn add_faucet(&mut self, name: String, f: Faucet) {
        self.faucet.insert(name, f);
    }

    pub fn add_junction(&mut self, name: String, j: Junction) {
        self.junction.insert(name, j);
    }

    pub fn add_buffer(&mut self, name: String, b: Buffer) {
        self.buffer.insert(name, b);
    }

    pub fn add_drain(&mut self, name: String, d: Drain) {
        self.drain.insert(name, d);
    }

    pub fn get_buffer_pull(&mut self, key: String) -> Result<Option<Pull>, ConnectableAddOutputError> {
        if let Some(x) = self.buffer.get_mut(&key) {
            return x.add_output(OutputPort::Out).map(|b| Some(b));
        }
        Ok(None)
    }

    pub fn get_faucet_pull(&mut self, key: String) -> Result<Option<Pull>, ConnectableAddOutputError> {
        if let Some(x) = self.faucet.get_mut(&key) {
            return x.add_output(OutputPort::Out).map(|f| Some(f));
        }
        Ok(None)
    }

    pub fn get_launch_pull(&mut self, key: String, port: OutputPort) -> Result<Option<Pull>, ConnectableAddOutputError> {
        if let Some(x) = self.launch.get_mut(&key) {
            return x.add_output(port).map(|l| Some(l));
        }
        Ok(None)
    }

    pub fn configure_faucet(&mut self, faucet_name: String, buffer_names: Vec<String>, low_watermark: usize, high_watermark: usize) -> bool {
        let has_all_buffers = buffer_names.iter().all(
            |f| self.buffer.get(f).is_some()
        );
        if !has_all_buffers || self.faucet.get(&faucet_name).is_none() {
            return false;
        }
        self.faucet_settings.insert(
            faucet_name,
            FaucetSettings { low_watermark, high_watermark, buffer_names }
        );
        true
    }

    fn faucet_settings(&mut self) -> Result<Vec<FaucetSettingsCapture>, WaiterError> {
        let mut faucet_settings_capture: Vec<FaucetSettingsCapture> = vec![];

        for (faucet_name, settings) in self.faucet_settings.iter() {
            let faucet = self.faucet.get_mut(faucet_name).ok_or_else(|| WaiterError::SettingsForMissingFaucet(faucet_name.to_string()))?;
            let faucet_component = faucet.get_control();
            let mut buffer_size_monitors = vec![];
            for buffer_name in settings.buffer_names.iter() {
                let buf = self.buffer.get_mut(buffer_name).ok_or_else(|| WaiterError::SettingsRefersMissingBuffer(buffer_name.to_string()))?;
                buffer_size_monitors.push(buf.add_buffer_size_monitor());
            }
            faucet_settings_capture.push(
                FaucetSettingsCapture {
                    low_watermark: settings.low_watermark,
                    high_watermark: settings.high_watermark,
                    faucet_component,
                    buffer_size_monitors
                }
            );
        }

        Ok(faucet_settings_capture)
    }

    #[allow(clippy::many_single_char_names)]
    pub async fn start(&mut self) -> Result<usize, WaiterError> {
        use futures::future::join_all;

        fn folder(acc: Result<usize, WaiterError>, cur: &mut Result<usize, WaiterError>) -> Result<usize, WaiterError> {
            match (acc, cur) {
                (Ok(i), Ok(j)) => Ok(i + *j),
                (Err(x), _) => Err(x),
                (_, x) => {
                    let mut y = Ok(0);
                    std::mem::swap(x, &mut y);
                    y
                }
            }
        }


        async fn start(name: &str, cntrl: &mut dyn StartableControl) -> Result<usize, WaiterError> {
            match cntrl.start().await {
                Err(e) => Err(WaiterError::CausedByError(name.to_string(), e)),
                Ok(x) => Ok(x),
            }
        }

        async fn start_launch(name: &str, cntrl: &mut LaunchString) -> Result<usize, WaiterError> {
            match cntrl.start().await {
                Err(e) => Err(WaiterError::CausedByError(name.to_string(), e)),
                Ok(x) => Ok(x),
            }
        }

        let managed_taps = join_all(self.faucet_settings()?.into_iter().map(manage_taps_from_buffers));
        let faucets = join_all(self.faucet.iter_mut().map(|(n, f)| start(n, f)));
        let launch = join_all(self.launch.iter_mut().map(
            |(n, l)| start_launch(n, l) )
        );
        let junction = join_all(self.junction.iter_mut().map(|(n, j)| start(n, j)));
        let buffers = join_all(self.buffer.iter_mut().map(|(n, b)| start(n, b)));
        let drain = join_all(self.drain.iter_mut().map(|(n, d)| start(n, d)));

        let (mut m, (mut f, (mut l, (mut j, (mut b, mut d))))) = managed_taps.join(faucets.join(launch.join(junction.join(buffers.join(drain))))).await;
        [
            m.iter_mut().fold(Ok(0), folder),
            f.iter_mut().fold(Ok(0), folder),
            l.iter_mut().fold(Ok(0), folder),
            j.iter_mut().fold(Ok(0), folder),
            b.iter_mut().fold(Ok(0), folder),
            d.iter_mut().fold(Ok(0), folder),
        ].iter_mut().fold(Ok(0), folder)
    }
}


struct CreateSpec<'a> {
    component_type: &'a ComponentType,
    component_name: &'a String,
}

fn get_create_spec(connection: &Connection) -> CreateSpec {
    match connection {
        Connection::MiddleConnection { component_type, component_name, .. } => CreateSpec { component_type, component_name },
        Connection::StartConnection { component_type, component_name, .. } => CreateSpec { component_type, component_name },
        Connection::EndConnection { component_type, component_name, .. } => CreateSpec { component_type, component_name },
    }
}

fn convert_connection_to_join_from(connection: &Connection) -> Option<JoinFrom> {
    match connection {
        Connection::MiddleConnection { component_type, component_name, output_port, .. } => Some(JoinFrom { component_type: *component_type, component_name: component_name, output_port: *output_port }),
        Connection::StartConnection { component_type, component_name, output_port } => Some(JoinFrom { component_type: *component_type, component_name: component_name, output_port: *output_port }),
        Connection::EndConnection { .. } => None,
    }
}

fn convert_connection_to_join_to(connection: &Connection) -> Option<JoinTo> {
    match connection {
        Connection::MiddleConnection { component_type, component_name, input_port: InputPort::In(input_priority), .. } => Some(JoinTo { component_type: *component_type, component_name, input_priority: *input_priority }),
        Connection::StartConnection { .. } => None,
        Connection::EndConnection { component_type, component_name, input_port: InputPort::In(input_priority) } => Some(JoinTo { component_type: *component_type, component_name, input_priority: *input_priority })
    }
}

pub fn get_waiter(mut config: Config) -> Result<Waiter, String> {

    let mut created: HashSet<(&ComponentType, &str)> = HashSet::new();
    let mut last: Option<&Connection> = None;

    let all_connections = config.connection.iter().fold(
        Vec::new(),
        |mut acc, (_hash_key, deser_conn)| {
            if let DeserializedConnection::Connections(v) = deser_conn {
                acc.extend_from_slice(v);
                return acc;
            }
            panic!("Encountered DeserializedConnection::JoinString in main::get_waiter()")
        }
    );

    let mut waiter = Waiter::new();

    async fn constructor(create_spec: &CreateSpec<'_>, config: &mut Config, w: &mut Waiter) -> Result<(), String> {

        match create_spec {
            CreateSpec { component_type: ComponentType::Faucet, component_name, .. } => {
                // TODO: Figure out how to get this in...
                let pull = match config.faucet.get(*component_name).map(|t| t.source.as_str()).unwrap_or("") {
                    "-" => Pull::Stdin(async_std::io::stdin(), ReadSplitControl::new()),
                    "" => Pull::None,
                    filename => {
                        let file = async_std::fs::File::open(filename).await.map_err(|_| { format!("Could not open file: {}", filename) })?;
                        Pull::File(file, ReadSplitControl::new())
                    },
                };
                let faucet = Faucet::new(pull);
                w.add_faucet(component_name.to_string(), faucet);
                Ok(())
            },
            CreateSpec { component_type: ComponentType::Drain, component_name, .. } => {
                // TODO: Figure out how to get this in...
                let push = match config.drain.get(*component_name).map(|s| s.destination.as_str()).unwrap_or("") {
                    "-" => Push::Stdout(async_std::io::stdout()),
                    "_" => Push::Stderr(async_std::io::stderr()),
                    "" => Push::None,
                    filename => {
                        let file = async_std::fs::File::create(filename).await.map_err(|_| { format!("Could not write to file: {}", filename) })?;
                        Push::File(async_std::io::BufWriter::new(file))
                    },
                };
                w.add_drain(component_name.to_string(), Drain::new(push));
                Ok(())
            },
            CreateSpec { component_type: ComponentType::Buffer, component_name, .. } => {
                w.add_buffer(component_name.to_string(), Buffer::new());
                Ok(())
            },
            CreateSpec { component_type: ComponentType::Junction, component_name, .. } => {
                w.add_junction(component_name.to_string(), Junction::new());
                Ok(())
            },
            CreateSpec { component_type: ComponentType::Launch, component_name, .. } => {
                if let Some(cfg) = config.launch.remove(*component_name) {
                    if cfg.command.is_none() {
                        return Err(format!("Launch '{}' did not have a command specified", component_name));
                    }
                    let launch: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
                        if cfg.env.is_empty() { None } else { Some(cfg.env) },
                        cfg.path,
                        cfg.command.ok_or(format!("Launch '{}' did not have a command specified", component_name))?,
                        if cfg.arg.is_empty() { None } else { Some(cfg.arg) }
                    );
                    w.add_launch(component_name.to_string(), launch);
                    return Ok(());
                }
                Err(format!("Could not find configuration for Launch {}", component_name))
            }
        }
    }


    for connection in all_connections.iter() {
        let create_spec = get_create_spec(connection);

        if !created.contains(&(create_spec.component_type, create_spec.component_name)) {
            async_std::task::block_on(constructor(&create_spec, &mut config, &mut waiter))?;
            created.insert((create_spec.component_type, create_spec.component_name));
        }

        if let Some(last_connection) = last {
            let err = match (convert_connection_to_join_from(last_connection), convert_connection_to_join_to(connection)) {
                (Some(join_component_from), Some(join_component_to)) => {
                    waiter.join(join_component_from, join_component_to).map_err(|c| format!("{}", c))
                },
                _ => Err(format!("There should have been a connection between {:?} and {:?}", last_connection, connection))
            };
            if let Err(err_msg) = err {
                return Err(err_msg);
            }
        }

        last = Some(connection);
        if let Connection::EndConnection { .. } = connection {
            last = None;
        }
    }

    for (faucet_name, FaucetConfig { source: _, monitored_buffers, buffered}) in config.faucet.into_iter() {
        if let Some(b) = buffered {
            waiter.configure_faucet(faucet_name, monitored_buffers, b.0, b.1);
        }
    }

    Ok(waiter)

}

