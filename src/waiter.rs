use crate::config::{Config, DeserializedConnection};
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


struct JoinFrom {
    pub component_id: usize,
    pub output_port: OutputPort,
}


struct JoinTo {
    pub component_id: usize,
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
    id: usize,
    component_type_name_to_id: HashMap<ComponentType, HashMap<String, usize>>,
    id_to_component_type_name: HashMap<usize, (ComponentType, String)>,
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
        let mut hm: HashMap<ComponentType, HashMap<String, usize>> = HashMap::new();
        hm.insert(ComponentType::Buffer, HashMap::new());
        hm.insert(ComponentType::Drain, HashMap::new());
        hm.insert(ComponentType::Faucet, HashMap::new());
        hm.insert(ComponentType::Junction, HashMap::new());
        hm.insert(ComponentType::Launch, HashMap::new());
        Waiter {
            faucet: HashMap::new(),
            launch: HashMap::new(),
            junction: HashMap::new(),
            buffer: HashMap::new(),
            drain: HashMap::new(),
            faucet_settings: HashMap::new(),
            id: 0,
            component_type_name_to_id: hm,
            id_to_component_type_name: HashMap::new(),
        }
    }


    fn incr_id(&mut self, component_type: ComponentType, name: String) -> usize {
        self.id_to_component_type_name.insert(self.id, (component_type, name.clone()));
        let hm = self.component_type_name_to_id.get_mut(&component_type).unwrap();
        let inner_entry = hm.entry(name.clone());
        inner_entry.or_insert(self.id);
        self.id = self.id + 1;
        self.id - 1
    }


    fn get_id(&self, component_type: &ComponentType, component_name: &str) -> Option<usize> {
        self.component_type_name_to_id.get(component_type).map(|hm| hm.get(component_name).map(|v| *v)).flatten()
    }


    fn get_src_pull(&mut self, src_id: usize, output_port: OutputPort) -> Result<Pull, ConnectableError> {

        let r = match self.id_to_component_type_name.get(&src_id) {
            Some((ComponentType::Faucet, component_name)) => {
                self.faucet.get_mut(component_name).map(|x| x.add_output(output_port))
            },
            Some((ComponentType::Launch, component_name)) => {
                self.launch.get_mut(component_name).map(|x| x.add_output(output_port))
            },
            Some((ComponentType::Junction, component_name)) => {
                self.junction.get_mut(component_name).map(|x| x.add_output(output_port))
            },
            Some((ComponentType::Buffer, component_name)) => {
                self.buffer.get_mut(component_name).map(|x| x.add_output(output_port))
            },
            Some((ComponentType::Drain, component_name,)) => {
                self.drain.get_mut(component_name).map(|x| x.add_output(output_port))
            },
            None => None,
        };

        match r {
            None => Err(ConnectableError::CouldNotFindSourceComponent(src_id)),
            Some(Err(x)) => Err(ConnectableError::AddOutput(src_id, x)),
            Some(Ok(x)) => Ok(x),
        }
    }

    pub fn join(&mut self, (src_id, output_port) : (usize, OutputPort), (dst_id, input_port) : (usize, InputPort)) -> Result<(), ConnectableError> {

        let pull = self.get_src_pull(src_id, output_port)?;
        let input_priority = match input_port {
            InputPort::In(n) => n
        };

        let res = match self.id_to_component_type_name.get(&dst_id) {
            Some((ComponentType::Faucet, component_name)) => {
                self.faucet.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
            Some((ComponentType::Launch, component_name)) => {
                self.launch.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
            Some((ComponentType::Junction, component_name)) => {
                self.junction.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
            Some((ComponentType::Buffer, component_name)) => {
                self.buffer.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
            Some((ComponentType::Drain, component_name)) => {
                self.drain.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
            },
            None => None,
        };

        match res {
            None => Err(ConnectableError::CouldNotFindDestinationComponent(dst_id)),
            Some(Err(x)) => Err(ConnectableError::AddInput(dst_id, x)),
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


#[derive(Debug)]
struct CreateSpec {
    component_type: ComponentType,
    component_name: String,
    input_port: Option<InputPort>,
    output_port: Option<OutputPort>,
    is_end_connection: bool,
}

fn get_create_spec(connection: Connection) -> CreateSpec {
    match connection {
        Connection::MiddleConnection { component_type, component_name, input_port, output_port } => CreateSpec {
            component_type,
            component_name,
            input_port: Some(input_port),
            output_port: Some(output_port),
            is_end_connection: false,
        },
        Connection::StartConnection { component_type, component_name, output_port } => CreateSpec {
            component_type,
            component_name,
            input_port: None,
            output_port: Some(output_port),
            is_end_connection: false,
        },
        Connection::EndConnection { component_type, component_name, input_port  } => CreateSpec {
            component_type,
            component_name,
            input_port: Some(input_port),
            output_port: None,
            is_end_connection: true,
        },
    }
}


pub fn get_waiter(mut config: Config) -> Result<Waiter, String> {

    let mut config_connections: HashMap<String, DeserializedConnection> = HashMap::new();
    std::mem::swap(&mut config.connection, &mut config_connections);
    let all_connections = config_connections.into_iter().fold(
        Vec::new(),
        |mut acc, (_hash_key, deser_conn)| {
            if let DeserializedConnection::Connections(mut v) = deser_conn {
                acc.append(&mut v);
                return acc;
            }
            panic!("Encountered DeserializedConnection::JoinString in waiter::get_waiter()")
        }
    );

    let mut waiter = Waiter::new();

    async fn constructor(component_type: ComponentType, component_name: String, config: &mut Config, w: &mut Waiter) -> Result<usize, String> {

        let id = w.incr_id(component_type, component_name.clone());
        match component_type {
            ComponentType::Faucet => {
                // TODO: Figure out how to get this in...
                let pull = match config.faucet.get(&component_name).map(|t| t.source.as_str()).unwrap_or("") {
                    "-" => Pull::Stdin(id, async_std::io::stdin(), ReadSplitControl::new()),
                    "" => Pull::None,
                    filename => {
                        let file = async_std::fs::File::open(filename).await.map_err(|_| { format!("Could not open file: {}", filename) })?;
                        Pull::File(id, file, ReadSplitControl::new())
                    },
                };
                let faucet = Faucet::new(id, pull);
                w.add_faucet(component_name.to_string(), faucet);
                Ok(id)
            },
            ComponentType::Drain => {
                // TODO: Figure out how to get this in...
                let push = match config.drain.get(&component_name).map(|s| s.destination.as_str()).unwrap_or("") {
                    "-" => Push::Stdout(id, async_std::io::stdout()),
                    "_" => Push::Stderr(id, async_std::io::stderr()),
                    "" => Push::None,
                    filename => {
                        let file = async_std::fs::File::create(filename).await.map_err(|_| { format!("Could not write to file: {}", filename) })?;
                        Push::File(id, async_std::io::BufWriter::new(file))
                    },
                };
                w.add_drain(component_name.to_string(), Drain::new(id, push));
                Ok(id)
            },
            ComponentType::Buffer => {
                w.add_buffer(component_name.to_string(), Buffer::new(id));
                Ok(id)
            },
            ComponentType::Junction => {
                w.add_junction(component_name.to_string(), Junction::new(id));
                Ok(id)
            },
            ComponentType::Launch => {
                if let Some(cfg) = config.launch.remove(&component_name) {
                    if cfg.command.is_none() {
                        return Err(format!("Launch '{}' did not have a command specified", component_name));
                    }
                    let launch: Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String> = Launch::new(
                        id,
                        if cfg.env.is_empty() { None } else { Some(cfg.env) },
                        cfg.path,
                        cfg.command.ok_or(format!("Launch '{}' did not have a command specified", component_name))?,
                        if cfg.arg.is_empty() { None } else { Some(cfg.arg) }
                    );
                    w.add_launch(component_name.to_string(), launch);
                    return Ok(id);
                }
                Err(format!("Could not find configuration for Launch {}", component_name))
            }
        }
    }


    let mut last: Option<(usize, OutputPort)> = None;
    for connection in all_connections.into_iter() {

        let mut create_spec = get_create_spec(connection);

        let dst_id = match waiter.get_id(&create_spec.component_type, &create_spec.component_name) {
            None => {
                let mut dst_component_type = ComponentType::Buffer;
                let mut dst_component_name = "".to_string();
                std::mem::swap(&mut create_spec.component_name, &mut dst_component_name);
                std::mem::swap(&mut create_spec.component_type, &mut dst_component_type);
                async_std::task::block_on(constructor(dst_component_type, dst_component_name, &mut config, &mut waiter))?
            },
            Some(id) => id
        };

        if let Some((src_id, src_output_port)) = last {

            let mut dst_input_port = Some(InputPort::In(0));
            std::mem::swap(&mut create_spec.input_port, &mut dst_input_port);

            if let Some(dst_input_port) = dst_input_port {
                waiter.join((src_id, src_output_port), (dst_id, dst_input_port)).map_err(|e| format!("{:?}", e))?;
            } else {
                panic!("HOW!");
            }
        }

        last = None;
        if let Some(create_spec_output_port) = create_spec.output_port {
            last = Some((dst_id, create_spec_output_port));
        }
    }

    for (faucet_name, FaucetConfig { source: _, monitored_buffers, buffered}) in config.faucet.into_iter() {
        if let Some(b) = buffered {
            waiter.configure_faucet(faucet_name, monitored_buffers, b.0, b.1);
        }
    }

    Ok(waiter)

}


