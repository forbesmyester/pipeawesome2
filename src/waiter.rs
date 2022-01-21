use std::collections::BTreeMap;
use crate::connectable::Breakable;
use std::time::Instant;
use crate::config::{Config, DeserializedConnection};
use crate::config::RegulatorConfig;
use crate::motion::Pull;
use crate::config::ComponentType;
use crate::config::Connection;
use crate::connectable::ConnectableError;
use crate::connectable::{ Connectable, InputPort, OutputPort };
use crate::{buffer::{Buffer, BufferSizeMessage}, drain::Drain, faucet::Faucet, regulator::{Regulator, RegulatorControl}, junction::Junction, launch::Launch, motion::MotionError, startable_control::StartableControl};
use std::collections::HashMap;
use async_std::{channel::Receiver, prelude::*};

type LaunchString = Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String>;


#[derive(Debug)]
struct FaucetSettings {
    low_watermark: usize,
    high_watermark: usize,
    buffer_names: Vec<String>,
}


#[derive(Debug)]
struct RegulatorSettingsCapture {
    low_watermark: usize,
    high_watermark: usize,
    regulator_component: RegulatorControl,
    buffer_size_monitors: Vec<Receiver<BufferSizeMessage>>,
}


async fn manage_regulator_from_buffers(mut regulator_settings: RegulatorSettingsCapture) -> Result<usize, WaiterError> {

    let mut back_off = crate::back_off::BackOff::new();

    #[derive(Debug)]
    struct Bsm { pub buffer_size: usize, pub buffer_monitor: Receiver<BufferSizeMessage>, }

    let mut monitors: Vec<Bsm> = regulator_settings.buffer_size_monitors.into_iter().map(|bm| {
        Bsm {buffer_size: 0, buffer_monitor: bm }
    }).collect();

    #[derive(Debug, PartialEq)]
    enum RegulatorStatus {
        Open,
        Closed,
    }

    let mut read_count = 0;
    let mut regulator_status = RegulatorStatus::Open;
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
        if regulator_status == RegulatorStatus::Open && total >= regulator_settings.high_watermark {
            regulator_settings.regulator_component.pause().await.map_err(|_| WaiterError::CouldNotPause)?;
            regulator_status = RegulatorStatus::Closed;
        }
        if regulator_status == RegulatorStatus::Closed && total <= regulator_settings.low_watermark {
            regulator_settings.regulator_component.resume().await.map_err(|_| WaiterError::CouldNotResume)?;
            regulator_status = RegulatorStatus::Open;
        }
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
    regulator: HashMap<String, Regulator>,
    regulator_settings: HashMap<String, FaucetSettings>,
    id: usize,
    component_type_name_to_id: HashMap<ComponentType, HashMap<String, usize>>,
    component_type_name_to_id_reverse: HashMap<usize, (ComponentType, String)>,
}


#[derive(Debug)]
pub enum WaiterError {
    CausedByError(ComponentType, String, MotionError),
    SettingsForMissingRegulator(String),
    SettingsRefersMissingBuffer(String, String),
    CouldNotPause,
    CouldNotResume,
}


impl WaiterError {

    pub fn description(&self) -> String {
        match self {
            WaiterError::CausedByError(_, _, m) => format!("{}", m),
            WaiterError::SettingsForMissingRegulator(s) => format!("There are no settings for Faucet:{}", s),
            WaiterError::SettingsRefersMissingBuffer(f, b) => format!("Faucet {} refers to buffer {} which does not exist", f, b),
            WaiterError::CouldNotPause => "CouldNotPause".to_string(),
            WaiterError::CouldNotResume => "CouldNotPause".to_string(),
        }
    }


    fn instant(&self) -> Option<&Instant> {
        match self {
            WaiterError::CausedByError(_, _, e) => e.instant(),
            _ => None,
        }
    }

    pub fn caused_by_error_name(&self) -> Option<&str> {
        match self {
            WaiterError::CausedByError(_, n, _) => Some(n),
            _ => None,
        }
    }

    pub fn caused_by_error_source(&self) -> Option<(&ComponentType, &String)> {
        match self {
            WaiterError::CausedByError(t, n, _) => Some((t, n)),
            _ => None,
        }
    }

    pub fn caused_by_error(&self) -> Option<&MotionError> {
        match self {
            WaiterError::CausedByError(_, _, e) => Some(e),
            _ => None,
        }
    }

}


#[allow(clippy::new_without_default)]
impl Waiter {

    pub fn new() -> Waiter {
        let mut hm: HashMap<ComponentType, HashMap<String, usize>> = HashMap::new();
        hm.insert(ComponentType::Buffer, HashMap::new());
        hm.insert(ComponentType::Drain, HashMap::new());
        hm.insert(ComponentType::Regulator, HashMap::new());
        hm.insert(ComponentType::Faucet, HashMap::new());
        hm.insert(ComponentType::Junction, HashMap::new());
        hm.insert(ComponentType::Launch, HashMap::new());
        Waiter {
            faucet: HashMap::new(),
            launch: HashMap::new(),
            junction: HashMap::new(),
            buffer: HashMap::new(),
            drain: HashMap::new(),
            regulator: HashMap::new(),
            regulator_settings: HashMap::new(),
            id: 0,
            component_type_name_to_id: hm,
            component_type_name_to_id_reverse: HashMap::new(),
        }
    }


    fn incr_id(&mut self, component_type: ComponentType, name: String) -> usize {
        self.component_type_name_to_id_reverse.insert(self.id, (component_type, name.clone()));
        let hm = self.component_type_name_to_id.get_mut(&component_type).unwrap();
        let inner_entry = hm.entry(name.clone());
        inner_entry.or_insert(self.id);
        self.id = self.id + 1;
        self.id - 1
    }


    fn get_id(&self, component_type: &ComponentType, component_name: &str) -> Option<usize> {
        self.component_type_name_to_id.get(component_type).map(|hm| hm.get(component_name).map(|v| *v)).flatten()
    }


    fn get_src_pull(&mut self, src_id: usize, dst_id: usize, output_port: OutputPort, breakable: Breakable) -> Result<Pull, ConnectableError> {

        let r = match self.component_type_name_to_id_reverse.get(&src_id) {
            Some((ComponentType::Faucet, component_name)) => {
                self.faucet.get_mut(component_name).map(|x| x.add_output(output_port, breakable, src_id, dst_id))
            },
            Some((ComponentType::Launch, component_name)) => {
                self.launch.get_mut(component_name).map(|x| x.add_output(output_port, breakable, src_id, dst_id))
            },
            Some((ComponentType::Junction, component_name)) => {
                self.junction.get_mut(component_name).map(|x| x.add_output(output_port, breakable, src_id, dst_id))
            },
            Some((ComponentType::Buffer, component_name)) => {
                self.buffer.get_mut(component_name).map(|x| x.add_output(output_port, breakable, src_id, dst_id))
            },
            Some((ComponentType::Drain, component_name,)) => {
                self.drain.get_mut(component_name).map(|x| x.add_output(output_port, breakable, src_id, dst_id))
            },
            Some((ComponentType::Regulator, component_name,)) => {
                self.regulator.get_mut(component_name).map(|x| x.add_output(output_port, breakable, src_id, dst_id))
            },
            None => None,
        };

        match r {
            None => Err(ConnectableError::CouldNotFindSourceComponent(src_id)),
            Some(Err(x)) => Err(ConnectableError::AddOutput(src_id, x)),
            Some(Ok(x)) => Ok(x),
        }
    }

    pub fn id_to_component_type_name(&self, id: &usize) -> Option<&(ComponentType, String)> {
        self.component_type_name_to_id_reverse.get(id)
    }

    pub fn join(&mut self, (src_id, output_port) : (usize, OutputPort), (dst_id, input_port) : (usize, InputPort)) -> Result<(), ConnectableError> {

        let pull = self.get_src_pull(src_id, dst_id, output_port, input_port.breakable)?;
        let input_priority = input_port.priority;

        let res = match self.component_type_name_to_id_reverse.get(&dst_id) {
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
            Some((ComponentType::Regulator, component_name)) => {
                self.regulator.get_mut(component_name).map(|x| x.add_input(pull, input_priority))
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

    pub fn add_regulator(&mut self, name: String, r: Regulator) {
        self.regulator.insert(name, r);
    }

    pub fn configure_regulator(&mut self, regulator_name: String, buffer_names: Vec<String>, low_watermark: usize, high_watermark: usize) -> bool {
        let has_all_buffers = buffer_names.iter().all(
            |f| self.buffer.get(f).is_some()
        );
        if !has_all_buffers || self.regulator.get(&regulator_name).is_none() {
            return false;
        }
        self.regulator_settings.insert(
            regulator_name,
            FaucetSettings { low_watermark, high_watermark, buffer_names }
        );
        true
    }

    fn regulator_settings(&mut self) -> Result<Vec<RegulatorSettingsCapture>, WaiterError> {
        let mut regulator_settings_capture: Vec<RegulatorSettingsCapture> = vec![];

        for (regulator_name, settings) in self.regulator_settings.iter() {
            let regulator = self.regulator.get_mut(regulator_name).ok_or_else(|| WaiterError::SettingsForMissingRegulator(regulator_name.to_string()))?;
            let regulator_component = regulator.get_control();
            let mut buffer_size_monitors = vec![];
            for buffer_name in settings.buffer_names.iter() {
                let buf = self.buffer.get_mut(buffer_name).ok_or_else(|| WaiterError::SettingsRefersMissingBuffer(regulator_name.to_string(), buffer_name.to_string()))?;
                buffer_size_monitors.push(buf.add_buffer_size_monitor());
            }
            regulator_settings_capture.push(
                RegulatorSettingsCapture {
                    low_watermark: settings.low_watermark,
                    high_watermark: settings.high_watermark,
                    regulator_component,
                    buffer_size_monitors
                }
            );
        }

        Ok(regulator_settings_capture)
    }

    #[allow(clippy::many_single_char_names)]
    pub async fn start(&mut self) -> Result<usize, Vec<WaiterError>> {
        use futures::future::join_all;

        let start_instant = Instant::now();

        fn folder(acc: Result<usize, Vec<WaiterError>>, cur: Result<usize, WaiterError>) -> Result<usize, Vec<WaiterError>> {
            match (acc, cur) {
                (Ok(i), Ok(j)) => Ok(i + j),
                (xs, Ok(_)) => xs,
                (Err(mut xs), Err(x)) => {
                    xs.push(x);
                    Err(xs)
                },
                (Ok(_), Err(x)) => Err(vec![x])
            }
        }


        async fn start(component_type: ComponentType, name: &str, cntrl: &mut dyn StartableControl) -> Result<usize, WaiterError> {
            match cntrl.start().await {
                Err(e) => Err(WaiterError::CausedByError(component_type, name.to_string(), e)),
                Ok(x) => Ok(x),
            }
        }

        async fn start_launch(name: &str, cntrl: &mut LaunchString) -> Result<usize, WaiterError> {
            match cntrl.start().await {
                Err(e) => Err(WaiterError::CausedByError(ComponentType::Launch, name.to_string(), e)),
                Ok(x) => Ok(x),
            }
        }

        fn err_as_vec_err(e: WaiterError) -> Vec<WaiterError> {
            vec![e]
        }

        let managed_regulators = join_all(self.regulator_settings().map_err(err_as_vec_err)?.into_iter().map(manage_regulator_from_buffers));
        let faucets = join_all(self.faucet.iter_mut().map(|(n, f)| start(ComponentType::Faucet, n, f)));
        let launch = join_all(self.launch.iter_mut().map(
            |(n, l)| start_launch(n, l) )
        );
        let junction = join_all(self.junction.iter_mut().map(|(n, j)| start(ComponentType::Junction, n, j)));
        let regulators = join_all(self.regulator.iter_mut().map(|(n, j)| start(ComponentType::Regulator, n, j)));
        let buffers = join_all(self.buffer.iter_mut().map(|(n, b)| start(ComponentType::Buffer, n, b)));
        let drain = join_all(self.drain.iter_mut().map(|(n, d)| start(ComponentType::Drain, n, d)));

        let (mut l, (mut r, (mut f, (mut m, (mut d, (mut b, mut j)))))) = launch.join(regulators.join(faucets.join(managed_regulators.join(drain.join(buffers.join(junction)))))).await;

        l.append(&mut f);
        l.append(&mut r);
        l.append(&mut m);
        l.append(&mut d);
        l.append(&mut b);
        l.append(&mut j);

        l.into_iter().fold(Ok(0), folder)
            .map_err(|mut v| {
                v.sort_by(|e1, e2| {
                    if let (Some(i1), Some(i2)) = (e1.instant(), e2.instant()) {
                        return i1.duration_since(start_instant).cmp(&i2.duration_since(start_instant));
                    }
                    std::cmp::Ordering::Equal
                });
                v
            })

    }
}


#[derive(Debug)]
struct CreateSpec {
    component_type: ComponentType,
    component_name: String,
    input_port: Option<InputPort>,
    output_port: Option<OutputPort>,
}

fn get_create_spec(connection: Connection) -> CreateSpec {
    match connection {
        Connection::MiddleConnection { component_type, component_name, input_port, output_port, .. } => CreateSpec {
            component_type,
            component_name,
            input_port: Some(input_port),
            output_port: Some(output_port),
        },
        Connection::StartConnection { component_type, component_name, output_port, .. } => CreateSpec {
            component_type,
            component_name,
            input_port: None,
            output_port: Some(output_port),
        },
        Connection::EndConnection { component_type, component_name, input_port, .. } => CreateSpec {
            component_type,
            component_name,
            input_port: Some(input_port),
            output_port: None,
        },
    }
}


pub fn get_waiter(mut config: Config) -> Result<Waiter, String> {

    let mut config_connections: BTreeMap<String, DeserializedConnection> = BTreeMap::new();
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
                let faucet = Faucet::new(config.faucet.get(&component_name).map(|t| t.source.clone()).unwrap_or("".to_string()));
                w.add_faucet(component_name.to_string(), faucet);
                Ok(id)
            },
            ComponentType::Drain => {
                // TODO: Figure out how to get this in...
                w.add_drain(component_name.to_string(), Drain::new(config.drain.get(&component_name).map(|s| s.destination.clone()).unwrap_or("".to_string())));
                Ok(id)
            },
            ComponentType::Regulator => {
                // TODO: Figure out how to get this in...
                w.add_regulator(component_name.to_string(), Regulator::new());
                Ok(id)
            },
            ComponentType::Buffer => {
                w.add_buffer(component_name.to_string(), Buffer::new());
                Ok(id)
            },
            ComponentType::Junction => {
                w.add_junction(component_name.to_string(), Junction::new());
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

            let mut dst_input_port = Some(InputPort { priority: 0, breakable: Breakable::Terminate });
            std::mem::swap(&mut create_spec.input_port, &mut dst_input_port);

            fn string_hack(id: &usize, o: Option<&(ComponentType, String)>) -> String {
                match o {
                    Some((ct, n)) => format!("{}:{}", ct, n),
                    None => format!("Unknown Component {}", id)
                }
            }

            if let Some(dst_input_port) = dst_input_port {
                match waiter.join((src_id, src_output_port), (dst_id, dst_input_port)) {
                    Ok(_) => Ok(()),
                    Err(ConnectableError::AddInput(src_id, connectable_add_input_error)) => {
                        Err(format!("AddInput({}, {:?})", string_hack(&src_id, waiter.id_to_component_type_name(&src_id)), connectable_add_input_error))
                    },
                    Err(ConnectableError::AddOutput(dst_id, connectable_add_output_error)) => {
                        Err(format!("AddOutput({}, {:?})", string_hack(&dst_id, waiter.id_to_component_type_name(&dst_id)), connectable_add_output_error))
                    },
                    Err(ConnectableError::CouldNotFindSourceComponent(src_id)) => {
                        Err(format!("ConnectableError::CouldNotFindSourceComponent({})", string_hack(&src_id, waiter.id_to_component_type_name(&src_id))))
                    },
                    Err(ConnectableError::CouldNotFindDestinationComponent(dst_id)) => {
                        Err(format!("ConnectableError::CouldNotFindDestinationComponent({})", string_hack(&dst_id, waiter.id_to_component_type_name(&dst_id))))
                    },
                }?;
            } else {
                panic!("HOW!");
            }
        }

        last = None;
        if let Some(create_spec_output_port) = create_spec.output_port {
            last = Some((dst_id, create_spec_output_port));
        }
    }

    for (regulator_name, RegulatorConfig { monitored_buffers, buffered}) in config.regulator.into_iter() {
        if let Some(b) = buffered {
            waiter.configure_regulator(regulator_name, monitored_buffers, b.0, b.1);
        }
    }

    Ok(waiter)

}


