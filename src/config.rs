use std::collections::HashSet;
use std::collections::HashMap;

use peg::{error::ParseError, str::LineCol};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputPort {
    Err,
    Out,
    Exit,
    Size
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
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
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
struct LaunchConfig {
    command: Option<String>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default = "HashMap::new")]
    env: HashMap<String, String>,
    #[serde(default = "Vec::new")]
    arg: Vec<String>,
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct FaucetConfig {
    min_buffered: usize,
    max_buffered: usize,
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

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct FlowConfig {
    #[serde(default = "HashMap::new")]
    faucet: HashMap<String, FaucetConfig>,
    #[serde(default = "HashMap::new")]
    launch: HashMap<String, LaunchConfig>,
    #[serde(default = "HashMap::new")]
    connection: HashMap<String, DeserializedConnection>,
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct ExecutionConfig {
    #[serde(default = "HashMap::new")]
    faucet: HashMap<String, String>,
    #[serde(default = "HashMap::new")]
    drain: HashMap<String, String>,
}

impl ExecutionConfig {
    pub fn new() -> ExecutionConfig {
        ExecutionConfig {
            faucet: HashMap::new(),
            drain: HashMap::new(),
        }
    }
}


impl Default for ExecutionConfig {
    fn default() -> Self {
        Self::new()
    }
}


impl FlowConfig {
    pub fn new() -> FlowConfig {
        FlowConfig {
            faucet: HashMap::new(),
            launch: HashMap::new(),
            connection: HashMap::new(),
        }
    }
}


impl Default for FlowConfig {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "ExecutionConfig::new")]
    execution: ExecutionConfig,
    #[serde(default = "FlowConfig::new")]
    flow: FlowConfig,
}


impl Config {
    pub fn faucet_set_watermark(mut config: Config, faucet_id: String, min: usize, max: usize) -> Config {
        let mut faucet_config = match min > max {
            true => FaucetConfig { min_buffered: max, max_buffered: min },
            false => FaucetConfig { min_buffered: min, max_buffered: max },
        };
        config.flow.faucet.entry(faucet_id)
            .and_modify(|x| std::mem::swap(x, &mut faucet_config))
            .or_insert(faucet_config);
        config
    }

    pub fn faucet_set_source(mut config: Config, faucet_id: String, mut faucet_source: String) -> Config {
        config.execution.faucet.entry(faucet_id)
            .and_modify(|x| std::mem::swap(x, &mut faucet_source))
            .or_insert(faucet_source);
        config
    }

    pub fn drain_set_destination(mut config: Config, drain_id: String, mut drain_destination: String) -> Config {
        config.execution.drain.entry(drain_id)
            .and_modify(|x| std::mem::swap(x, &mut drain_destination))
            .or_insert(drain_destination);
        config
    }

    pub fn connection_join(mut config: Config, connection_id: String, mut connection_spec: DeserializedConnection) -> Config {
        config.flow.connection.entry(connection_id)
            .and_modify(|x| std::mem::swap(x, &mut connection_spec))
            .or_insert(connection_spec);
        config
    }

    pub fn launch_set_command(mut config: Config, launch_id: String, command: String) -> Config {
        config.flow.launch.entry(launch_id)
            .and_modify(|mut lc| lc.command = Some(command.clone()))
            .or_insert(LaunchConfig { command: Some(command), path: None, env: HashMap::new(), arg: vec![] });
        config
    }
 pub fn launch_set_path(mut config: Config, launch_id: String, path: String) -> Config {
        config.flow.launch.entry(launch_id)
            .and_modify(|mut lc| lc.path = Some(path.clone()))
            .or_insert(LaunchConfig { command: None, path: Some(path), env: HashMap::new(), arg: vec![] });
        config
    }

    pub fn launch_set_args(mut config: Config, launch_id: String, mut args: Vec<String>) -> Config {
        config.flow.launch.entry(launch_id)
            .and_modify(|mut lc| lc.arg = std::mem::take(&mut args))
            .or_insert(LaunchConfig { command: None, path: None, env: HashMap::new(), arg: args });
        config
    }

    pub fn launch_set_env(mut config: Config, launch_id: String, mut env: HashMap<String, String>) -> Config {
        config.flow.launch.entry(launch_id)
            .and_modify(|mut lc| lc.env = std::mem::take(&mut env))
            .or_insert(LaunchConfig { command: None, path: None, env, arg: vec![] });
        config
    }

    // TODO: Rewrite this because we shouldn't need ownership, so this is pure laziness!
    pub fn lint(mut config: Config) -> Vec<String> {
        use std::iter::FromIterator;

        let mut r: Vec<String> = vec![];

        let registered: HashMap<String, HashSet<String>> = HashMap::<_, _>::from_iter([
            ("execution.faucet".to_string(), config.execution.faucet.iter().map(|x| x.0.to_string()).collect::<HashSet<String>>()),
            ("execution.drain".to_string(), config.execution.drain.iter().map(|x| x.0.to_string()).collect::<HashSet<String>>()),
            ("flow.faucet".to_string(), config.flow.faucet.iter().map(|x| x.0.to_string()).collect::<HashSet<String>>()),
            ("flow.launch".to_string(), config.flow.launch.iter().map(|x| x.0.to_string()).collect::<HashSet<String>>()),
        ]);

        fn mapper(kv: (std::string::String, DeserializedConnection)) -> (String, Result<Vec<Connection>, String>) {
            let v = match kv.1 {
                DeserializedConnection::JoinString(s) => match load_connection_from_string(&s) {
                    Err(_) => Err(format!("Could not parse connection {} ({})", kv.0, s)),
                    Ok(x) => Ok(x)
                },
                DeserializedConnection::Connections(x) => Ok(x),
            };
            (kv.0, v)
        }

        let connections_as_vec = std::mem::take(&mut config.flow.connection).into_iter()
            .map(mapper)
            .filter_map(|(k, v)| {
                match v {
                    Ok(vv) => Some((k, vv)),
                    Err(s) => {
                        r.push(s);
                        None
                    }
                }
            })
            .collect::<HashMap<String, Vec<Connection>>>();

        let known_components: HashSet<(&ComponentType, &str)> = connections_as_vec.iter()
            .fold(
                HashSet::new(),
                |mut acc, (_, v_conn)| {
                    for conn in v_conn {
                        let cn = match conn {
                            Connection::EndConnection { component_type, component_name, .. } => (component_type, component_name),
                            Connection::MiddleConnection { component_type, component_name, .. } => (component_type, component_name),
                            Connection::StartConnection { component_type, component_name, .. } => (component_type, component_name),
                        };
                        acc.insert((cn.0, cn.1));
                    }
                    acc
                }
            );

        r.append(
            registered.iter().fold(
                &mut Vec::new(),
                |acc, (reg_key, reg_controls)| {
                    let mut to_add: Vec<String> = reg_controls.iter()
                        .filter(|reg_control| !known_components.contains(&(&ComponentType::Faucet, reg_control as &str)))
                        .map(|reg_control| format!("Config was expecting a {} called {}, but none exists", reg_key, reg_control))
                        .collect();
                    acc.append(&mut to_add);
                    acc
                }
            )
        );


        fn exists_with_flow_config(registered: &HashMap<std::string::String, std::collections::HashSet<std::string::String>>, registered_key: &str, component_name: &str) -> bool {
            registered.get(registered_key).and_then(|hs| {
                if hs.contains(component_name) { return Some(true); }
                None
            }).is_none()
        }

        r.append(
                known_components.iter().fold(
                &mut Vec::new(),
                |acc, (component_type, component_name)| {
                    let in_connections_but_missing_reqd_config = match component_type {
                        ComponentType::Launch => {
                            config.flow.launch.get(*component_name).and_then(|l| l.command.as_ref()).is_none()
                        },
                        ComponentType::Drain => {
                            exists_with_flow_config(&registered, "execution.drain", *component_name)
                            // registered.get("execution.faucet").and_then(|hs| {
                            //     if hs.contains(*component_name) { return Some(true); }
                            //     None
                            // }).is_none()
                        },
                        ComponentType::Faucet => {
                            exists_with_flow_config(&registered, "execution.drain", *component_name)
                        },
                        _ => false,
                    };
                    if in_connections_but_missing_reqd_config {
                        acc.push(format!(
                            "Connections references a {}:{} but it is missing required settings",
                            component_type,
                            component_name
                        ));
                    }
                    acc
                }
            )
        );

        r
    }

    pub fn new() -> Config {
        Config {
            flow: FlowConfig { faucet: HashMap::new(), launch: HashMap::new(), connection: HashMap::new() },
            execution: ExecutionConfig { faucet: HashMap::new(), drain: HashMap::new() }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn config_serde() {

    assert_eq!(
        serde_json::from_str::<FlowConfig>(r#"{"connection": {"a": "faucet[O] | [3]drain"}}"#).unwrap(),
        FlowConfig {
            faucet: HashMap::new(),
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
        serde_json::from_str::<FlowConfig>(r#"{
            "launch": {
                "command_1": { "command": "cat" },
                "command_2": { "command": "cat", "env": { "USER": "forbesmyester" }, "arg": ["-n"], "path": "/home/forbesmyester" }
            },
            "faucet": {
                "tap": {
                    "max_buffered": 1000,
                    "min_buffered": 500
                }
            },
            "connection": {
                "ynmds": [ { "component_type": "faucet", "component_name": "tap", "output_port": "out" }, { "component_type": "launch", "component_name": "command_1", "output_port": "out", "input_port": "in", "priority": 3 }],
                "trfxg": "command_1 | command_2",
                "oojza": "command_1[S] | tap",
                "ynbhz": [ { "component_type": "launch", "component_name": "command_2", "output_port": "out", "input_port": "in", "priority": 3 }, { "component_type": "drain", "component_name": "drain", "input_port": "in", "priority": 3 } ]
            }
        }"#).unwrap(),
        FlowConfig {
                faucet: HashMap::<_, _>::from_iter([("tap".to_string(), FaucetConfig { max_buffered: 1000, min_buffered: 500 })]),
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
                        "oojza".to_string(),
                        DeserializedConnection::JoinString("command_1[S] | tap".to_string())
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
            flow: FlowConfig {
                faucet: HashMap::new(),
                launch: HashMap::new(),
                connection: HashMap::new(),
            },
            execution: ExecutionConfig {
                faucet: HashMap::new(),
                drain: HashMap::new(),
            }
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
                = "[" p:$(['E'|'O'|'X'|'S']) "]" {
                    match p {
                        "S" => OutputPort::Size,
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

