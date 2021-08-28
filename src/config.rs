use peg::{error::ParseError, str::LineCol};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputPort {
    Err,
    Out,
    Exit,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    Faucet,
    Launch,
    Junction,
    Buffer,
    Drain,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
//#[serde(tag = "input_port", content = "priority")]
#[serde(rename_all = "lowercase")]
// #[serde(untagged)]
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
pub enum DeserializedConnections {
    String(String),
    Connections(Vec<Connection>),
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Config {
    connections: Vec<DeserializedConnections>,
}


#[test]
fn config_serde() {

    assert_eq!(
        serde_json::from_str::<Config>(r#"{"connections": ["faucet[O] | [3]drain"]}"#).unwrap(),
        Config { connections: vec![ DeserializedConnections::String("faucet[O] | [3]drain".to_string()) ] }
    );

    println!("TO STRING: {}", serde_json::to_string(
        &Config { connections: vec![
            DeserializedConnections::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "faucet".to_string(), output_port: OutputPort::Out },
                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "x".to_string(), input_port: InputPort::In(5) }
            ])
        ] }
    ).unwrap());

    assert_eq!(
        serde_json::from_str::<Connection>(r#"{"component_type": "drain", "component_name": "x", "input_port": "in", "priority": 5}"#).unwrap(),
        Connection::EndConnection { component_type: ComponentType::Drain, component_name: "x".to_string(), input_port: InputPort::In(5) }
    );

    assert_eq!(
        serde_json::from_str::<Config>(r#"{
            "connections": [
                [ { "component_type": "faucet", "component_name": "faucet", "output_port": "out" }, { "component_type": "launch", "component_name": "command_1", "output_port": "out", "input_port": "in", "priority": 3 }],
                "command_1 | command_2",
                [ { "component_type": "launch", "component_name": "command_2", "output_port": "out", "input_port": "in", "priority": 3 }, { "component_type": "drain", "component_name": "drain", "input_port": "in", "priority": 3 } ]
            ]
        }"#).unwrap(),
        Config {
                connections: vec![
                DeserializedConnections::Connections(vec![
                    Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "faucet".to_string(), output_port: OutputPort::Out },
                    Connection::MiddleConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Out, input_port: InputPort::In(3) },
                ]),
                DeserializedConnections::String("command_1 | command_2".to_string()),
                DeserializedConnections::Connections(vec![
                    Connection::MiddleConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, input_port: InputPort::In(3) },
                    Connection::EndConnection { component_type: ComponentType::Drain, component_name: "drain".to_string(), input_port: InputPort::In(3) }
                ])
            ]
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
                = t:$("f" / "d" / "j" / "b" / "l") {
                    match t {
                        "f" => ComponentType::Faucet,
                        "d" => ComponentType::Drain,
                        "j" => ComponentType::Junction,
                        "b" => ComponentType::Buffer,
                        "l" => ComponentType::Launch,
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

            pub rule connection_set() -> Vec<Connection>
                = s:component_start() pipe() m:line_middle()* e:component_end() {
                    let mut r = vec![s];
                    r.extend_from_slice(&m);
                    r.extend_from_slice(&[e]);
                    r
                }

        }
    }

    connection_parser::connection_set(s)

}


#[test]
fn test_load_connection_from_string() {

    assert_eq!(
        load_connection_from_string("f:faucet[O] | [22]l:command[E] | b:x | l:y[O] | [-2]d:drain").unwrap(),
        vec![
            Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "faucet".to_string(), output_port: OutputPort::Out },
            Connection::MiddleConnection { input_port: InputPort::In(22), component_type: ComponentType::Launch, component_name: "command".to_string(), output_port: OutputPort::Err },
            Connection::MiddleConnection { input_port: InputPort::In(0), component_type: ComponentType::Buffer, component_name: "x".to_string(), output_port: OutputPort::Out },
            Connection::MiddleConnection { input_port: InputPort::In(0), component_type: ComponentType::Launch, component_name: "y".to_string(), output_port: OutputPort::Out },
            Connection::EndConnection { input_port: InputPort::In(-2), component_type: ComponentType::Drain, component_name: "drain".to_string() },
        ]
    );

    assert_eq!(
        // load_connection_from_string("f:faucet[O] | [3]d:drain").unwrap(),
        load_connection_from_string("f:faucet[O] | [3]d:drain").unwrap(),
        vec![
            Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "faucet".to_string(), output_port: OutputPort::Out },
            Connection::EndConnection { input_port: InputPort::In(3), component_type: ComponentType::Drain, component_name: "drain".to_string() },
        ]
    );


}

