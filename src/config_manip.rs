use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;

use crate::config::ComponentType;
use crate::config::Config;
use crate::config::Connection;
use crate::config::DeserializedConnection;
use crate::connectable::Breakable;
use crate::connectable::InputPort;
use crate::connectable::OutputPort;


fn generate_new_key(original_key: &str, existing_keys: &HashSet<String>) -> String {
    let mut i = 1;
    loop {
        let new_key = format!("{}_{}", original_key, i);
        if !existing_keys.contains(&new_key) {
            return new_key;
        }
        i = i + 1;
    }
}

enum ApartIterator {
    Single(Connection),
    Double(Connection, Connection)
}

fn break_apart_chunker(mut acc: Vec<ApartIterator>, item: Connection) -> Vec<ApartIterator> {

    let mut to_add = match acc.pop() {
        None => vec![ApartIterator::Single(item)],
        Some(ApartIterator::Single(first_item)) => {
            vec![ApartIterator::Double(first_item, item)]
        },
        Some(ApartIterator::Double(first_item, second_item)) => {

            let new_start = second_item.clone();

            vec![
                ApartIterator::Double(first_item, second_item),
                ApartIterator::Double(new_start, item),
            ]
        }
    };
    acc.append(&mut to_add);
    acc

}

fn break_apart_mapper(item: ApartIterator) -> ApartIterator {

    match item {
        ApartIterator::Single(_) => { panic!("break_apart_mapper encountered None"); },
        ApartIterator::Double(start, end) => {
            let new_end = match end {
                Connection::EndConnection { component_type, component_name, input_port, connection_set } => {
                    Connection::EndConnection { component_type, component_name, input_port, connection_set }
                },
                Connection::MiddleConnection { component_type, component_name, input_port, connection_set, ..  } => {
                    Connection::EndConnection { component_type, component_name, input_port, connection_set }
                },
                Connection::StartConnection { .. } => {
                    panic!("Last item in ApartIterator::Double is a StartConnection");
                }
            };

            let new_start = match start {
                Connection::EndConnection {..} => {
                    panic!("Last item in ApartIterator::Double is a EndConnection");
                },
                Connection::MiddleConnection { component_type, component_name , output_port, connection_set, .. } => {
                    Connection::StartConnection { component_type, component_name, output_port, connection_set }
                },
                Connection::StartConnection { component_type, component_name, output_port, connection_set } => {
                    Connection::StartConnection { component_type, component_name, output_port, connection_set }
                }
            };

            ApartIterator::Double(new_start, new_end)

        },
    }

}

fn apart_iterator_to_double_connection(ai: ApartIterator) -> DeserializedConnection {
    match ai {
        ApartIterator::Single(_) => {
            panic!("apart_iterator_to_double_connection encountered ApartIterator::Single");
        },
        ApartIterator::Double(first_item, second_item) => {
            DeserializedConnection::Connections(vec![first_item, second_item])
        }
    }
}

fn break_apart(input: Vec<Connection>) -> Vec<DeserializedConnection> {
    input
        .into_iter()
        .fold(vec![], break_apart_chunker)
        .into_iter()
        .map(break_apart_mapper)
        .map(apart_iterator_to_double_connection).collect()
}


/// Converts the connections so that each DeserializedConnection is only of 2 length
pub fn rebuild_pair_up_connections_folder(mut acc: BTreeMap<String, DeserializedConnection>, (item_k, item_v): (String, DeserializedConnection)) -> BTreeMap<String, DeserializedConnection> {
    let mut existing_keys = acc.keys().map(|x| x.clone()).collect();
    match item_v {
        DeserializedConnection::JoinString(_) => { panic!("rebuild_joiner_folder encountered DeserializedConnection::JoinString"); }
        DeserializedConnection::Connections(v) => {
            let pairs = break_apart(v);
            for p in pairs {
                let new_key = generate_new_key(&item_k, &existing_keys);
                existing_keys.insert(new_key.clone());
                acc.insert(new_key, p);
            }
        }
    };
    acc
}


#[test]
fn test_pair_up_connections_folder() {

    use std::iter::FromIterator;
    use crate::connectable::InputPort;
    use crate::connectable::Breakable;

    let input: BTreeMap::<String, DeserializedConnection> = BTreeMap::<_, _>::from_iter([
        (
            "main".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::MiddleConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Out, input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::MiddleConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        )
    ]);

    let broken: Vec<DeserializedConnection> = vec![
        DeserializedConnection::Connections(vec![
            Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
            Connection::EndConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
        ]),
        DeserializedConnection::Connections(vec![
            Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, connection_set: None },
            Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
        ])
    ];

    assert_eq!(
        broken,
        break_apart(vec![
            Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
            Connection::MiddleConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
            Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
        ])
    );

    let expected: BTreeMap<String, DeserializedConnection> = BTreeMap::<_, _>::from_iter([
        (
            "main_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "main_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "ynbhz_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        )
    ]);

    assert_eq!(expected, input.into_iter().fold(BTreeMap::new(), rebuild_pair_up_connections_folder));

}


fn count_start_ends(in_config: &BTreeMap::<String, DeserializedConnection>) -> (HashMap<(ComponentType, String, OutputPort), usize>, HashMap<(ComponentType, String), usize>) {
    in_config.iter().fold(
        (HashMap::new(), HashMap::new()),
        | (mut outcount, mut incount) , (_, item) | {
            let vec = match &item {
                DeserializedConnection::Connections(v) => v,
                DeserializedConnection::JoinString(_) => {
                    panic!("add_junctions::in_counts - Has JoinString");
                }
            };
            let (start, end) = match vec.as_slice() {
                [Connection::StartConnection { component_type: ctstart, component_name: start, output_port, .. }, Connection::EndConnection { component_type: ctend, component_name: end, .. }] => {
                    ((*ctstart, start.to_string(), output_port.clone()), (*ctend, end.to_string()))
                }
                _ => { panic!("add_junctions::in_counts - Not start end pair"); }
            };
            *outcount.entry(start).or_insert(0) += 1;
            *incount.entry(end).or_insert(0) += 1;
            (outcount, incount)
        }
    )
}


fn extract_junction_names(connections: &BTreeMap<String, DeserializedConnection>) -> HashSet<String> {

    connections.values().fold(
        HashSet::new(),
        |mut acc, item| {

            fn junction_names_filter_map(c: &Connection) -> Option<String> {
                match c {
                    Connection::EndConnection { component_type, component_name, .. } => {
                        if *component_type == ComponentType::Junction {
                            return Some(component_name.clone());
                        }
                        None
                    },
                    Connection::MiddleConnection { .. } => {
                        panic!("junction_names - Has Middle");
                    },
                    Connection::StartConnection { component_type, component_name, .. } => {
                        if *component_type == ComponentType::Junction {
                            return Some(component_name.clone());
                        }
                        None
                    }
                }
            }

            let vec = match &item {
                DeserializedConnection::Connections(v) => {
                    v.iter().filter_map(junction_names_filter_map)
                },
                DeserializedConnection::JoinString(_) => {
                    panic!("junction_names - Has JoinString");
                }
            };

            acc.extend(vec);
            acc
        }
    )

}


fn get_multis(mut acc: Vec<(ComponentType, String)>, (k, v): ((ComponentType, String), usize)) -> Vec<(ComponentType, String)> {
    if v < 2 { return acc; }
    acc.push(k);
    acc
}


fn get_multi_starts(mut acc: Vec<(ComponentType, String, OutputPort)>, (k, v): ((ComponentType, String, OutputPort), usize)) -> Vec<(ComponentType, String, OutputPort)> {
    if v < 2 { return acc; }
    acc.push(k);
    acc
}


/// Launch (etc) only has one input, but in config it can have multiple. This will add a junction before it to handle discrepency
pub fn add_junctions(connections: BTreeMap<String, DeserializedConnection>) -> BTreeMap<String, DeserializedConnection> {

    let (start_counts, end_counts) = count_start_ends(&connections);
    let mut existing_keys: HashSet<String> = connections.keys().map(|x| x.clone()).collect();

    fn add_start_junctions_for(existing_keys: &mut HashSet<String>, multi_start: (ComponentType, String, OutputPort), in_config: BTreeMap::<String, DeserializedConnection>) -> BTreeMap::<String, DeserializedConnection> {

        let junction_name = generate_new_key(&"junction", &extract_junction_names(&in_config));
        let mut added_to_junction = false;
        in_config.into_iter().fold(BTreeMap::new(), |mut acc, (k, dsc)| {
            let mut dsc_vec = match dsc {
                DeserializedConnection::Connections(v) => v,
                DeserializedConnection::JoinString(_) => {
                    panic!("add_start_junctions_for - Has JoinString");
                }
            };
            if dsc_vec.len() != 2 {
                panic!("add_start_junctions_for - does not have 2 elements");
            }
            let end = dsc_vec.pop();
            let start = dsc_vec.pop();
            match (start, end) {
                (Some(Connection::StartConnection { component_type, component_name, output_port, .. }), Some(Connection::EndConnection { component_type: end_component_type, component_name: end_component_name, input_port: end_input_port, .. })) if (component_type == multi_start.0) && (component_name == multi_start.1) && (output_port == multi_start.2) => {

                    if !added_to_junction {
                        let left_start = Connection::StartConnection { component_type, component_name, output_port, connection_set: None };
                        let left_end = Connection::EndConnection { component_type: ComponentType::Junction, component_name: junction_name.clone(), input_port: InputPort { breakable: Breakable::Finish, priority: end_input_port.priority }, connection_set: None };
                        let new_key = generate_new_key(&"dyn_connection", existing_keys);
                        existing_keys.insert(new_key.clone());
                        acc.insert(
                            new_key,
                            DeserializedConnection::Connections(vec![left_start, left_end])
                        );
                        added_to_junction = true;
                    }

                    let right_start = Connection::StartConnection { component_type: ComponentType::Junction, component_name: junction_name.clone(), output_port: OutputPort::Out, connection_set: None };
                    let right_end = Connection::EndConnection { component_type: end_component_type, component_name: end_component_name, input_port: end_input_port, connection_set: None };

                    acc.insert(k, DeserializedConnection::Connections(vec![right_start, right_end]));
                    acc
                }
                (Some(start), Some(end)) => {
                    acc.insert(k, DeserializedConnection::Connections(vec![start, end]));
                    acc
                }
                _ => { panic!("add_start_junctions_for - unexpected"); }
            }
        })
    }

    fn add_end_junctions_for(existing_keys: &mut HashSet<String>, multi_end: (ComponentType, String), in_config: BTreeMap::<String, DeserializedConnection>) -> BTreeMap::<String, DeserializedConnection> {

        let junction_name = generate_new_key(&"junction", &extract_junction_names(&in_config));
        let mut added_to_junction = false;
        in_config.into_iter().fold(BTreeMap::new(), |mut acc, (k, dsc)| {
            let mut dsc_vec = match dsc {
                DeserializedConnection::Connections(v) => v,
                DeserializedConnection::JoinString(_) => {
                    panic!("add_start_junctions_for - Has JoinString");
                }
            };
            if dsc_vec.len() != 2 {
                panic!("add_start_junctions_for - does not have 2 elements");
            }
            let end = dsc_vec.pop();
            let start = dsc_vec.pop();
            match (start, end) {
                (Some(Connection::StartConnection { component_type, component_name, output_port, .. }), Some(Connection::EndConnection { component_type: end_component_type, component_name: end_component_name, input_port: end_input_port, .. })) if (end_component_type == multi_end.0) && (end_component_name == multi_end.1) => {

                    let left_start = Connection::StartConnection { component_type, component_name, output_port, connection_set: None };
                    let left_end = Connection::EndConnection { component_type: ComponentType::Junction, input_port: end_input_port.clone(), component_name: junction_name.clone(), connection_set: None };

                    acc.insert(k, DeserializedConnection::Connections(vec![left_start, left_end]));

                    if !added_to_junction {
                        let right_start = Connection::StartConnection { component_type: ComponentType::Junction, component_name: junction_name.clone(), output_port: OutputPort::Out, connection_set: None };
                        let right_end = Connection::EndConnection { component_type: end_component_type, component_name: end_component_name, input_port: InputPort { breakable: Breakable::Finish, priority: end_input_port.priority }, connection_set: None };
                        let new_key = generate_new_key(&"dyn_connection", existing_keys);
                        existing_keys.insert(new_key.clone());
                        acc.insert(
                            new_key,
                            DeserializedConnection::Connections(vec![right_start, right_end])
                        );
                        added_to_junction = true;
                    }

                    acc
                }
                (Some(start), Some(end)) => {
                    acc.insert(k, DeserializedConnection::Connections(vec![start, end]));
                    acc
                }
                _ => { panic!("add_start_junctions_for - unexpected"); }
            }
        })
    }

    let multi_starts: Vec<(ComponentType, String, OutputPort)> = start_counts.into_iter().fold(Vec::new(), get_multi_starts);
    let multi_ends: Vec<(ComponentType, String)> = end_counts.into_iter().fold(Vec::new(), get_multis);

    let start_done = multi_starts.into_iter().fold(
        connections,
        |acc, multi_start| { add_start_junctions_for(&mut existing_keys, multi_start, acc ) }
    );

    multi_ends.into_iter().fold(
        start_done,
        |acc, multi_end| { add_end_junctions_for(&mut existing_keys, multi_end, acc ) }
    )

}

#[test]
fn test_add_junctions() {

    use std::iter::FromIterator;
    use crate::connectable::InputPort;
    use crate::connectable::Breakable;

    let input: BTreeMap<String, DeserializedConnection> = BTreeMap::<_, _>::from_iter([
        (
            "main_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "main_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "ynbhz_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "a_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Err, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Launch, component_name: "command_3".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "a_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_3".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        )
    ]);

    let (start_counts, end_counts) = count_start_ends(&input);

    assert_eq!(
        start_counts,
        HashMap::<_, _>::from_iter([
            ((ComponentType::Faucet, "tap".to_string(), OutputPort::Out), 2),
            ((ComponentType::Faucet, "tap".to_string(), OutputPort::Err), 1),
            ((ComponentType::Launch, "command_1".to_string(), OutputPort::Out), 1),
            ((ComponentType::Launch, "command_2".to_string(), OutputPort::Out), 1),
            ((ComponentType::Launch, "command_3".to_string(), OutputPort::Out), 1),
        ])
    );
    assert_eq!(
        end_counts,
        HashMap::<_, _>::from_iter([
            ((ComponentType::Drain, "sinkhole".to_string()), 3),
            ((ComponentType::Launch, "command_1".to_string()), 1),
            ((ComponentType::Launch, "command_2".to_string()), 1),
            ((ComponentType::Launch, "command_3".to_string()), 1),
        ])
    );

   let expected: BTreeMap<String, DeserializedConnection> = BTreeMap::<_, _>::from_iter([
        (
            "dyn_connection_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_1".to_string(), input_port: InputPort { breakable: Breakable::Finish, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "dyn_connection_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "sinkhole".to_string(), input_port: InputPort { breakable: Breakable::Finish, priority: 3 }, connection_set: None },
            ]),
        ),        (
            "main_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Junction, component_name: "junction_1".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ]),
        ),
                (
            "main_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Junction, component_name: "junction_1".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "ynbhz_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "a_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Err, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Launch, component_name: "command_3".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "a_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_3".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        )
    ]);

    assert_eq!(add_junctions(input), expected);


}


fn identify_required_spies_for_launch_exits(connections: &BTreeMap<String, DeserializedConnection>) -> HashSet<String> {

    let connection_pairs: Vec<(Connection, Connection)> = connections.values().filter_map(|dsc| {
        match dsc {
            DeserializedConnection::Connections(v) if v.len() != 2 => {
                None
            },
            DeserializedConnection::Connections(v) => {
                Some((v[0].clone(), v[1].clone()))
            },
            _ => None,
        }
    }).collect();

    let launch_has_exit_already: HashSet<String> = connection_pairs.iter().filter_map(|(first, _)| {
        match first {
            Connection::StartConnection { component_type, component_name, output_port, .. }
                if (component_type == &ComponentType::Launch) && (output_port == &OutputPort::Exit) => {
                    Some(component_name.clone())
                },
            _ => None,
        }
    }).collect();

    let all_launch = connection_pairs.into_iter().fold(HashSet::new(), |mut acc, (first, _last)| {
        match first {
            Connection::StartConnection { component_type, component_name, .. }
                if component_type == ComponentType::Launch => {
                    acc.insert(component_name.clone());
                    acc
                },
            _ => acc,
        }
    });

    all_launch.into_iter().filter_map(|name| {
        match launch_has_exit_already.contains(&name) {
            true => None,
            false => Some(name)
        }
    }).collect()

}



#[test]
fn test_identify_required_spies_for_launch_exits() {

    use std::iter::FromIterator;
    use crate::connectable::InputPort;
    use crate::connectable::Breakable;

    let input: BTreeMap<String, DeserializedConnection> = BTreeMap::<_, _>::from_iter([
        (
            "dyn_connection_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_1".to_string(), input_port: InputPort { breakable: Breakable::Finish, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "main_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Exit, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_3".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_3".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_3".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
    ]);

    let expected: HashSet<String> = HashSet::from_iter(vec!["command_1".to_owned(), "command_3".to_owned()]);

    assert_eq!(
        identify_required_spies_for_launch_exits(&input),
        expected
    );

}

fn add_launch_exit_spies(config: &mut Config, to_add: Vec<String>) {

    let mut existing_connection_keys = config.connection.keys().map(|x| x.clone()).collect();
    let mut existing_drains = config.drain.keys().map(|x| x.clone()).collect();
    println!("{:?}", existing_drains);

    for launch_name in to_add {
        let new_drain_key = generate_new_key("dyn_drain", &existing_drains);
        existing_drains.insert(new_drain_key.clone());
        let new_connection_key = generate_new_key("dyn_connection", &existing_connection_keys);
        existing_connection_keys.insert(new_connection_key.clone());
        config.drain.insert(new_drain_key.clone(), crate::config::DrainConfig { destination: "/dev/null".to_owned() });
        config.connection.insert(new_connection_key, DeserializedConnection::Connections(vec![
            Connection::StartConnection { component_type: ComponentType::Launch, component_name: launch_name.to_owned(), output_port: OutputPort::Exit, connection_set: None },

            Connection::EndConnection { component_type: ComponentType::Drain, component_name: new_drain_key, input_port: InputPort { breakable: Breakable::Terminate, priority: 0 }, connection_set: None }
        ]));
    }
}


#[test]
fn test_add_launch_exit_spies() {

    use std::iter::FromIterator;
    use crate::connectable::InputPort;
    use crate::connectable::Breakable;

    let input_connections: BTreeMap<String, DeserializedConnection> = BTreeMap::<_, _>::from_iter([
        (
            "dyn_connection_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_1".to_string(), input_port: InputPort { breakable: Breakable::Finish, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "main_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Exit, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_3".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_3".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_3".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
    ]);

    let expected_connections: BTreeMap<String, DeserializedConnection> = BTreeMap::<_, _>::from_iter([
        (
            "dyn_connection_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Faucet, component_name: "tap".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_1".to_string(), input_port: InputPort { breakable: Breakable::Finish, priority: 3 }, connection_set: None },
            ]),
        ),
        (
            "main_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Out, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_1".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_2".to_string(), output_port: OutputPort::Exit, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "ynbhz_3".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_3".to_string(), output_port: OutputPort::Out, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Junction, component_name: "junction_3".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 3 }, connection_set: None }
            ])
        ),
        (
            "dyn_connection_2".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_1".to_string(), output_port: OutputPort::Exit, connection_set: None },
                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "dyn_drain_2".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 0 }, connection_set: None }
            ])
        ),
        (
            "dyn_connection_3".to_string(),
            DeserializedConnection::Connections(vec![
                Connection::StartConnection { component_type: ComponentType::Launch, component_name: "command_3".to_string(), output_port: OutputPort::Exit, connection_set: None },

                Connection::EndConnection { component_type: ComponentType::Drain, component_name: "dyn_drain_3".to_string(), input_port: InputPort { breakable: Breakable::Terminate, priority: 0 }, connection_set: None }
            ])
        ),
    ]);

    let mut config = Config::new();
    config.connection = input_connections;
    config.drain.insert("dyn_drain_1".to_owned(), DrainConfig { destination: "/dev/null".to_owned() });

    let resulting_config = add_launch_exit_spies(&mut config, vec!["command_1".to_owned(), "command_3".to_owned()]);

    assert_eq!(config.connection, expected_connections);
    assert_eq!(
        config.drain.keys().collect::<HashSet<&String>>(),
        HashSet::from_iter(vec![&"dyn_drain_1".to_owned(), &"dyn_drain_2".to_owned(), &"dyn_drain_3".to_owned()])
    );

}



pub fn connection_manipulation(mut config: Config) -> Config {
    let mut read_config_connections = std::mem::take(&mut config.connection);
    Config::convert_connections(&mut read_config_connections);
    let paired_config_connections = read_config_connections.into_iter().fold(BTreeMap::new(), rebuild_pair_up_connections_folder);
    let mut manipulated_connections = add_junctions(paired_config_connections);
    let launch_missing_exit_spies = identify_required_spies_for_launch_exits(&manipulated_connections).into_iter().collect();
    std::mem::swap(&mut config.connection, &mut manipulated_connections);
    add_launch_exit_spies(&mut config, launch_missing_exit_spies);
    config
}

pub fn connection_manipulation_light(mut config: Config) -> Config {
    let mut read_config_connections = std::mem::take(&mut config.connection);
    Config::convert_connections(&mut read_config_connections);
    let mut paired_config_connections = read_config_connections.into_iter().fold(BTreeMap::new(), rebuild_pair_up_connections_folder);
    std::mem::swap(&mut config.connection, &mut paired_config_connections);
    config
}
