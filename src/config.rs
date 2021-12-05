use std::collections::HashSet;
use std::collections::HashMap;

use peg::{error::ParseError, str::LineCol};
use serde::{Deserialize, Serialize};
use crate::connectable::OutputPort;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    Faucet,
    Launch,
    Junction,
    Buffer,
    Drain,
}


impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct LaunchConfig {
    pub command: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default = "HashMap::new")]
    pub env: HashMap<String, String>,
    #[serde(default = "Vec::new")]
    pub arg: Vec<String>,
}


fn default_faucet_drain_config_source_destination() -> String {
    "-".to_string()
}


fn default_faucet_empty_vector() -> Vec<String> {
    vec![]
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct FaucetConfig {
    pub buffered: Option<(usize, usize)>,
    #[serde(default = "default_faucet_empty_vector")]
    pub monitored_buffers: Vec<String>,
    #[serde(default = "default_faucet_drain_config_source_destination")]
    pub source: String,
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct DrainConfig {
    #[serde(default = "default_faucet_drain_config_source_destination")]
    pub destination: String,
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "input_port", content = "priority")]
pub enum InputPort {
    In(isize),
}

struct ComponentIdentifier ( ComponentType, String );

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Connection {
    MiddleConnection {
        component_type: ComponentType,
        component_name: String,
        #[serde(flatten)]
        input_port: InputPort,
        output_port: OutputPort,
    },
    StartConnection{
        component_type: ComponentType,
        component_name: String,
        output_port: OutputPort,
    },
    EndConnection {
        component_type: ComponentType,
        component_name: String,
        #[serde(flatten)]
        input_port: InputPort,
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DeserializedConnection {
    JoinString(String),
    Connections(Vec<Connection>),
}


impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "HashMap::new")]
    pub faucet: HashMap<String, FaucetConfig>,
    #[serde(default = "HashMap::new")]
    pub drain: HashMap<String, DrainConfig>,
    #[serde(default = "HashMap::new")]
    pub launch: HashMap<String, LaunchConfig>,
    #[serde(default = "HashMap::new")]
    pub connection: HashMap<String, DeserializedConnection>,
}


#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum ConfigLintWarning {
    InConfigButMissingFlowConnection { config_section: String, component_type: ComponentType, component_name: String },
    InFlowConnectionButMissingConfig { component_type: ComponentType, component_name: String },
    InvalidConnection { id: String, join: String },
}

impl ToString for ConfigLintWarning {


    fn to_string(&self) -> std::string::String {
        match self {
            ConfigLintWarning::InConfigButMissingFlowConnection { config_section, component_type, component_name } => {
                format!(
                    "PA: Error: 0001: {}:{} - Configuration exists in {}, but this is not referenced in the connections",
                    component_type,
                    component_name,
                    config_section
                )
            },
            ConfigLintWarning::InFlowConnectionButMissingConfig { component_type, component_name } => {
                format!(
                    "PA: Error: 0002: {}:{} - Is referenced in the flow connections, but is missing required configuration",
                    component_type,
                    component_name,
                )
            },
            ConfigLintWarning::InvalidConnection { id, join } => {
                format!("PA: Error: 0003: {}:{} - Could not be converted from the configuration", id, join)
            },
        }
    }
}


impl Config {

    pub fn quick_deserialized_connection_to_connection(ds: &DeserializedConnection) -> Vec<&Connection> {
        match ds {
            DeserializedConnection::Connections(cs) => {
                let mut v = vec![];
                for c in cs { v.push(c); }
                v
            },
            DeserializedConnection::JoinString(_) => vec![],
        }
    }

    pub fn faucet_set_watermark(mut config: Config, faucet_id: String, min: usize, max: usize) -> Config {
        let faucet_config = match min > max {
            true => FaucetConfig { source: default_faucet_drain_config_source_destination(), buffered: Some((max, min)), monitored_buffers: vec![] },
            false => FaucetConfig { source: default_faucet_drain_config_source_destination(), buffered: Some((min, max)), monitored_buffers: vec![] },
        };
        config.faucet.entry(faucet_id)
            .and_modify(|x| {
                x.buffered = faucet_config.buffered;
            })
            .or_insert(faucet_config);
        config
    }

    pub fn faucet_set_source(mut config: Config, faucet_id: String, faucet_source: String) -> Config {
        config.faucet.entry(faucet_id)
            .and_modify(|x| { x.source = faucet_source.to_string() })
            .or_insert(FaucetConfig { source: faucet_source, buffered: None, monitored_buffers: vec![] });
        config
    }

    pub fn drain_set_destination(mut config: Config, drain_id: String, drain_destination: String) -> Config {
        let mut d = DrainConfig { destination: drain_destination };
        config.drain.entry(drain_id)
            .and_modify(|x| std::mem::swap(x, &mut d))
            .or_insert(d);
        config
    }

    pub fn connection_join(mut config: Config, connection_id: String, mut connection_spec: DeserializedConnection) -> Config {
        config.connection.entry(connection_id)
            .and_modify(|x| std::mem::swap(x, &mut connection_spec))
            .or_insert(connection_spec);
        config
    }

    pub fn launch_set_command(mut config: Config, launch_id: String, command: String) -> Config {
        config.launch.entry(launch_id)
            .and_modify(|mut lc| lc.command = Some(command.clone()))
            .or_insert(LaunchConfig { command: Some(command), path: None, env: HashMap::new(), arg: vec![] });
        config
    }
    pub fn launch_set_path(mut config: Config, launch_id: String, path: String) -> Config {
        config.launch.entry(launch_id)
            .and_modify(|mut lc| lc.path = Some(path.clone()))
            .or_insert(LaunchConfig { command: None, path: Some(path), env: HashMap::new(), arg: vec![] });
        config
    }

    pub fn launch_set_args(mut config: Config, launch_id: String, mut args: Vec<String>) -> Config {
        config.launch.entry(launch_id)
            .and_modify(|mut lc| lc.arg = std::mem::take(&mut args))
            .or_insert(LaunchConfig { command: None, path: None, env: HashMap::new(), arg: args });
        config
    }

    pub fn launch_set_env(mut config: Config, launch_id: String, mut env: HashMap<String, String>) -> Config {
        config.launch.entry(launch_id)
            .and_modify(|mut lc| lc.env = std::mem::take(&mut env))
            .or_insert(LaunchConfig { command: None, path: None, env, arg: vec![] });
        config
    }

    pub fn connections_mapper<'a>((k, v): (&'a String, &'a String)) -> Result<Vec<Connection>,(&'a str, &'a str)> {

        match load_connection_from_string(v) {
            Err(_) => Err((k, v)),
            Ok(x) => Ok(x),
        }

    }

    pub fn convert_connections(connections: &mut HashMap<String, DeserializedConnection>) -> HashSet<ConfigLintWarning> {

        connections.iter_mut()
            .filter_map(
                |(k, v)| {
                    let replace_with = match v {
                        DeserializedConnection::JoinString(s) => {
                            match Config::connections_mapper((k, s)) {
                                Ok(x) => Some(x),
                                Err((kk, vv)) => {
                                    return Some(ConfigLintWarning::InvalidConnection { id: kk.to_string(), join: vv.to_string() });
                                }
                            }
                        },
                        DeserializedConnection::Connections(_) => None
                    };
                    if let Some(x) = replace_with {
                        let mut d = DeserializedConnection::Connections(x);
                        std::mem::swap(v, &mut d);
                    };
                    None
                })
            .collect()

    }

    pub fn lint(config: &mut Config) -> HashSet<ConfigLintWarning> {
        use std::iter::FromIterator;

        let mut errs = Config::convert_connections(&mut config.connection);
        if !errs.is_empty() {
            return errs;
        }

        fn string_to_component_type(s: &str) -> ComponentType {
            match s {
                "faucet" => ComponentType::Faucet,
                "launch" => ComponentType::Launch,
                "faucet[_].monitored_buffers" => ComponentType::Buffer,
                "drain" => ComponentType::Drain,
                _ => panic!("Encountered known reg_key in in Config::lint()"),
            }
        }

        let registered: HashMap<String, HashSet<String>> = HashMap::<_, _>::from_iter([
            ("drain".to_string(), config.drain.iter().map(|x| x.0.to_string()).collect::<HashSet<String>>()), // recommended (where to output)
            ("faucet".to_string(), config.faucet.iter().map(|x| x.0.to_string()).collect::<HashSet<String>>()), // optional (min/max buffered)
            (
                "faucet[_].monitored_buffers".to_string(),
                config.faucet.iter().fold(
                    HashSet::new(),
                    |mut acc, kv| {
                        for mb in kv.1.monitored_buffers.iter() {
                            acc.insert(mb.to_string());
                        }
                        acc
                    }
                )
            ), // optional (min/max buffered)
            ("launch".to_string(), config.launch.iter().map(|x| x.0.to_string()).collect::<HashSet<String>>()), // required (how to launch the programs)
        ]);

        // Collect the names / types of all components in the flow of data
        let known_components: HashSet<(&ComponentType, &str)> = config.connection.iter()
            .fold(
                HashSet::new(),
                |mut acc, (_, deserialized_connection)| {
                    for conn in Config::quick_deserialized_connection_to_connection(deserialized_connection) {
                        let cn = match conn {
                            Connection::EndConnection { component_type, component_name, .. } => (component_type, component_name),
                            Connection::MiddleConnection { component_type, component_name, .. } => (component_type, component_name),
                            Connection::StartConnection { component_type, component_name, .. } => (component_type, component_name),
                        };
                        acc.insert((&cn.0, &cn.1));
                    }
                    acc
                }
            );

        // If something is registered, but does not have a corresponding connection... what do we care?
        registered.iter().fold(
            &mut errs,
            |acc, (reg_key, reg_controls)| {
                let reg_component_type = string_to_component_type(reg_key.as_str());
                for reg_control in reg_controls {
                    if known_components.contains(&(&reg_component_type, reg_control as &str)) {
                        continue;
                    }
                    // TODO: Check taps don't reference missing buffers
                    acc.insert(
                        ConfigLintWarning::InConfigButMissingFlowConnection {
                            component_type: string_to_component_type(reg_key),
                            component_name: reg_control.to_owned(),
                            config_section: reg_key.to_owned(),
                        }
                    );
                }
                acc
            }
        );

        fn exists_with_flow_config(registered: &HashMap<std::string::String, std::collections::HashSet<std::string::String>>, registered_key: &str, component_name: &str) -> bool {
            registered.get(registered_key).and_then(|hs| {
                if hs.contains(component_name) { return Some(true); }
                None
            }).is_none()
        }

        // If something is in the flow of data, but is missing required config we have a problem.
        known_components.iter().fold(
            &mut errs,
            |acc, (component_type, component_name)| {
                let in_connections_but_missing_reqd_config = match component_type {
                    ComponentType::Drain => {
                        exists_with_flow_config(&registered, "drain", *component_name)
                    },
                    ComponentType::Faucet => {
                        exists_with_flow_config(&registered, "faucet", *component_name)
                    },
                    ComponentType::Launch => {
                        exists_with_flow_config(&registered, "launch", *component_name)
                    },
                    _ => false,
                };
                if in_connections_but_missing_reqd_config {
                    acc.insert(
                        ConfigLintWarning::InFlowConnectionButMissingConfig {
                            component_type: **component_type,
                            component_name: component_name.to_owned().to_string()
                        }
                    );
                }
                acc
            }
        );

        errs
    }

    pub fn new() -> Config {
        Config {
            faucet: HashMap::new(),
            drain: HashMap::new(),
            launch: HashMap::new(),
            connection: HashMap::new(),
        }
    }


}


#[test]
fn config_serde() {

    use std::iter::FromIterator;

    assert_eq!(
        serde_json::from_str::<Config>(r#"{"connection": {"a": "faucet[O] | [3]drain"}}"#).unwrap(),
        Config {
            faucet: HashMap::new(),
            drain: HashMap::new(),
            launch: HashMap::new(),
            connection: HashMap::<_, _>::from_iter([
                ("a".to_string(), DeserializedConnection::JoinString("faucet[O] | [3]drain".to_string()))
            ])
        }
    );

    assert_eq!(
        serde_json::from_str::<Connection>(r#"{"component_type": "drain", "component_name": "x", "input_port": "in", "priority": 5}"#).unwrap(),
        Connection::EndConnection { component_type: ComponentType::Drain, component_name: "x".to_string(), input_port: InputPort::In(5) }
    );

    // pa --faucet-src=- --drain-dst=- --faucet-min-max tap100,1000 --launch-command command_1=cat --launch-command command_2=cat --launch-env command_2=USER=forbesmyester --launch-arg command_2=-n --launch-path command_2=/home/forbesmyester --connection 0='command1[S] | tap'

    assert_eq!(
        serde_json::from_str::<Config>(r#"{
            "launch": {
                "command_1": { "command": "cat" },
                "command_2": { "command": "cat", "env": { "USER": "forbesmyester" }, "arg": ["-n"], "path": "/home/forbesmyester" }
            },
            "faucet": {
                "tap": {
                    "buffered": [500, 1000],
                    "monitored_buffers": ["abc", "def"]
                }
            },
            "connection": {
                "ynmds": [ { "component_type": "faucet", "component_name": "tap", "output_port": "out" }, { "component_type": "launch", "component_name": "command_1", "output_port": "out", "input_port": "in", "priority": 3 }],
                "trfxg": "command_1 | command_2",
                "ynbhz": [ { "component_type": "launch", "component_name": "command_2", "output_port": "out", "input_port": "in", "priority": 3 }, { "component_type": "drain", "component_name": "drain", "input_port": "in", "priority": 3 } ]
            }
        }"#).unwrap(),
        Config {
                faucet: HashMap::<_, _>::from_iter([("tap".to_string(), FaucetConfig { source: "-".to_string(), buffered: Some((500, 1000)), monitored_buffers: vec!["abc".to_string(), "def".to_string()] })]),
                drain: HashMap::new(),
                launch: HashMap::<_, _>::from_iter([
                    ( "command_1".to_string(), LaunchConfig { command: Some("cat".to_string()), arg: vec![], path: None, env: HashMap::new() } ),
                    ( "command_2".to_string(), LaunchConfig {
                        command: Some("cat".to_string()),
                        arg: vec!["-n".to_string()],
                        path: Some("/home/forbesmyester".to_string()),
                        env: HashMap::<_, _>::from_iter([( "USER".to_string(), "forbesmyester".to_string() ) ])
                    } ),
                ]),
                connection: HashMap::<_, _>::from_iter([
                    (
                        "ynmds".to_string(),
                        DeserializedConnection::Connections(vec![
                            Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out },
                            Connection::MiddleConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Out, input_port: InputPort::In(3) },
                        ])
                    ),
                    (
                        "trfxg".to_string(),
                        DeserializedConnection::JoinString("command_1 | command_2".to_string())
                    ),
                    (
                        "ynbhz".to_string(),
                        DeserializedConnection::Connections(vec![
                            Connection::MiddleConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, input_port: InputPort::In(3) },
                            Connection::EndConnection { component_type: ComponentType::Drain, component_name: "drain".to_string(), input_port: InputPort::In(3) }
                        ])
                    )
                ])
        }
    );

    assert_eq!(
        serde_json::from_str::<Config>("{}").unwrap(),
        Config {
            faucet: HashMap::new(),
            launch: HashMap::new(),
            connection: HashMap::new(),
            drain: HashMap::new(),
        }
    );

}

pub fn load_connection_from_string(s: &str) -> Result<Vec<Connection>, ParseError<LineCol>> {

    peg::parser!{

        grammar connection_parser() for str {

            use self::Connection;

            rule component_name() -> String
                = s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]+) { s.to_string() }

            rule port_preference() -> isize
                = n:$("-"?['0'..='9']*) { n.parse().unwrap_or(0) }

            rule in_port() -> InputPort
                = "[" p:port_preference() "]" { InputPort::In(p) }

            rule component_type() -> ComponentType
                = t:$("faucet" / "drain" / "junction" / "buffer" / "launch" / "f" / "d" / "j" / "b" / "l") {
                    match t {
                        "f" => ComponentType::Faucet,
                        "d" => ComponentType::Drain,
                        "j" => ComponentType::Junction,
                        "b" => ComponentType::Buffer,
                        "l" => ComponentType::Launch,
                        "faucet" => ComponentType::Faucet,
                        "drain" => ComponentType::Drain,
                        "junction" => ComponentType::Junction,
                        "buffer" => ComponentType::Buffer,
                        "launch" => ComponentType::Launch,
                        _ => panic!("'{}' is not a valid component type", t)
                    }
                }

            rule identifier() -> ComponentIdentifier
                = t:component_type() component_type_name_seperator() n:component_name() { ComponentIdentifier(t, n) }

            rule component_type_name_seperator() -> bool
                = ":" { true }

            rule out_port() -> OutputPort
                = "[" p:$(['E'|'O'|'X']) "]" {
                    match p {
                        "X" => OutputPort::Exit,
                        "E" => OutputPort::Err,
                        "O" => OutputPort::Out,
                        _ => OutputPort::Out,
                    }
                }

            rule pipe() -> bool
                = " "* ("|" / "="+) " "* { true }

            rule component_middle_full() -> Connection
                = l:in_port() r:port_preference() i:identifier() o:out_port() {
                    Connection::MiddleConnection {
                        component_type: i.0,
                        component_name: i.1,
                        input_port: l,
                        output_port: o
                    }
                 }

            rule component_middle_default_input() -> Connection
                =  i:identifier() o:out_port() {
                    Connection::MiddleConnection {
                        component_type: i.0,
                        component_name: i.1,
                        input_port: InputPort::In(0),
                        output_port: o,
                    }
                }

            rule component_middle_quick() -> Connection
                = i:identifier() {
                    Connection::MiddleConnection {
                        component_type: i.0,
                        component_name: i.1,
                        input_port: InputPort::In(0),
                        output_port: OutputPort::Out,
                    }
                }

            rule component_middle()  -> Connection
                = x:( component_middle_full() / component_middle_default_input() / component_middle_quick() ) { x }

            rule component_start_full() -> Connection
                = i:identifier() o:out_port() {
                    Connection::StartConnection {
                        component_type: i.0,
                        component_name: i.1,
                        output_port: o
                    }
                }

            rule component_start_quick() -> Connection
                = i:identifier() {
                    Connection::StartConnection {
                        component_type: i.0,
                        component_name: i.1,
                        output_port: OutputPort::Out
                    }
                }

            rule component_start() -> Connection
                = c:component_start_full() / c: component_start_quick() { c }

            rule component_end_full()  -> Connection
                = l:in_port() r:port_preference() i:identifier() {
                    Connection::EndConnection {
                        component_type: i.0,
                        component_name: i.1,
                        input_port: l,
                    }
                }

            rule component_end_quick()  -> Connection
                = i:identifier() {
                    Connection::EndConnection {
                        component_type: i.0,
                        component_name: i.1,
                        input_port: InputPort::In(0),
                    }
                }

            rule component_end() -> Connection
                = c:( component_end_full() / component_end_quick() ) { c }

            rule line_middle() -> Connection
                = m:component_middle() pipe() {
                    m
                }

            pub rule connection_set_has() -> Vec<Connection>
                = s:component_start() pipe() m:line_middle()* e:component_end() {
                    let mut r = vec![s];
                    r.extend_from_slice(&m);
                    r.extend_from_slice(&[e]);
                    r
                }

            pub rule connection_set_none() -> Vec<Connection>
                = "" { vec![] }

            pub rule connection_set() -> Vec<Connection>
                = x:( connection_set_has() / connection_set_none() ) { x }

        }
    }

    connection_parser::connection_set(s)

}


#[test]
fn test_load_connection_from_string() {

    assert_eq!(
        load_connection_from_string("").unwrap(),
        vec![ ]
    );

    assert_eq!(
        load_connection_from_string("f:faucet[O] | [22]l:command[E] | buffer:x | l:y[O] | [-2]d:drain").unwrap(),
        vec![
            Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "faucet".to_string(), output_port: OutputPort::Out },
            Connection::MiddleConnection { input_port: InputPort::In(22), component_type: ComponentType::Launch, component_name: "command".to_string(), output_port: OutputPort::Err },
            Connection::MiddleConnection { input_port: InputPort::In(0), component_type: ComponentType::Buffer, component_name: "x".to_string(), output_port: OutputPort::Out },
            Connection::MiddleConnection { input_port: InputPort::In(0), component_type: ComponentType::Launch, component_name: "y".to_string(), output_port: OutputPort::Out },
            Connection::EndConnection { input_port: InputPort::In(-2), component_type: ComponentType::Drain, component_name: "drain".to_string() },
        ]
    );

    assert_eq!(
        load_connection_from_string("f:faucet[O] | [3]d:drain").unwrap(),
        vec![
            Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "faucet".to_string(), output_port: OutputPort::Out },
            Connection::EndConnection { input_port: InputPort::In(3), component_type: ComponentType::Drain, component_name: "drain".to_string() },
        ]
    );


}


#[test]
fn test_lint() {

    use std::iter::FromIterator;
    let config_string = r#"{
      "faucet": {
        "tap": { "source": "res/test/simple_input.txt" }
      },
      "drain": {
        "hole": { "destination": "-" },
        "plug": { "destination": "_" }
      },
      "launch": {
        "filter_adult": { "command": "grep", "path": null, "env": {}, "arg": [ "-v", "^[^ \\]\\{5\\}" ] },
        "mark_adult": { "command": "sed", "path": null, "env": {}, "arg": [ "s/^/Adult:/" ] },
        "exit_filter_adult": { "command": "awk", "path": null, "env": {}, "arg": [" { print \"EXIT: filter_adult: \" $0 }"] }
      },
      "connection": {
        "main": "f:tap | j:split | l:filter_adult | l:mark_adult | j:join | d:hole"
      }
    }"#;

    let mut config = serde_json::from_str::<Config>(config_string).unwrap();
    println!("{:?}", config);
    assert_eq!(
        HashSet::<_>::from_iter([
            ConfigLintWarning::InConfigButMissingFlowConnection { config_section: "execution.drain".to_string(), component_type: ComponentType::Drain, component_name: "plug".to_string() },
            ConfigLintWarning::InConfigButMissingFlowConnection { config_section: "flow.launch".to_string(), component_type: ComponentType::Launch, component_name: "exit_filter_adult".to_string() },
        ]),
        Config::lint(&mut config)
    );

}
