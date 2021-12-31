use crate::config::{ ComponentType, Connection };
use crate::connectable::OutputPort;
use std::collections::HashMap;

use graphviz_rust::dot_structures::*;
use graphviz_rust::dot_generator::*;
use graphviz_rust::printer::{PrinterContext,DotPrinter};


pub struct GraphConnection<'a> {
    src: (&'a ComponentType, &'a str, &'a Option<String>),
    via: &'a OutputPort,
    dst: (&'a ComponentType, &'a str, &'a Option<String>),
}

pub fn convert_connection_to_graph_connection<'a>(mut acc: Vec<GraphConnection<'a>>, connections: Vec<&'a Connection>) -> Vec<GraphConnection<'a>> {

    let mut src: Option<(&ComponentType, &str, Option<&OutputPort>, &Option<String>)> = None;
    let mut dst: Option<(&ComponentType, &str, Option<&OutputPort>, &Option<String>)>;
    let mut r: Vec<GraphConnection> = vec![];

    for connection in connections {
        dst = match connection {
            Connection::MiddleConnection { component_type, component_name, output_port, connection_set, .. } => Some((component_type, component_name, Some(output_port), connection_set)),
            Connection::StartConnection { component_type, component_name, output_port, connection_set } => Some((component_type, component_name, Some(output_port), connection_set)),
            Connection::EndConnection { component_type, component_name, connection_set, .. } => Some((component_type, component_name, None, connection_set)),
        };
        match (src, dst) {
            (Some((src_0, src_1, Some(src_2), connection_set)), Some(dst)) => {
                r.push(GraphConnection {
                    src: (src_0, src_1, connection_set),
                    via: src_2,
                    dst: (dst.0, dst.1, &dst.3),
                });
            },
            _ => (),
        }
        src = match dst {
            Some((dst_type, dst_name, Some(output_port), connection_set)) => Some((dst_type, dst_name, Some(output_port), connection_set)),
            _ => None,
        };
    }

    acc.append(&mut r);
    acc

}


pub fn convert_connection_components_fold<'a>(mut acc: HashMap<&'a ComponentType, Vec<&'a str>>, connections: Vec<&'a Connection>) -> HashMap<&'a ComponentType, Vec<&'a str>> {

    for connection in connections {
        let comp: (&ComponentType, &str) = match connection {
            Connection::MiddleConnection { component_type, component_name, .. } => (component_type, component_name),
            Connection::StartConnection { component_type, component_name, .. } => (component_type, component_name),
            Connection::EndConnection { component_type, component_name, .. } => (component_type, component_name),
        };

        acc.entry(comp.0)
            .and_modify(|x| {
                x.push(comp.1);
            })
            .or_insert(vec![comp.1]);
    }


    acc
}

pub fn get_graph(subgraph: Vec<Subgraph>) -> String {
    let mut stmts: Vec<Stmt> = vec![Stmt::Attribute(attr!("labeljust","l"))];
    let mut more_stmts = subgraph.into_iter().map(|sg| { Stmt::Subgraph(sg) }).collect();
    stmts.append(&mut more_stmts);

    let graph = Graph::DiGraph {
        id: Id::Plain("g_get_graph".to_string()),
        strict: true,
        stmts
    };

    let mut ctx = PrinterContext::default();
    graph.print(&mut ctx)
}


pub fn get_diagram(components: HashMap<&ComponentType, Vec<&str>>, connections: Vec<GraphConnection>) -> Subgraph {

    fn component_type_to_letter(ct: &ComponentType) -> &str {
        match ct {
            ComponentType::Buffer => "b",
            ComponentType::Drain => "d",
            ComponentType::Faucet => "f",
            ComponentType::Junction => "j",
            ComponentType::Launch => "l",
        }
    }

    fn get_node_shape(component_type: &ComponentType) -> Attribute {
        match component_type {
            ComponentType::Faucet => attr!("shape","trapezium"),
            ComponentType::Drain => attr!("shape","invtrapezium"),
            ComponentType::Junction => attr!("shape","oval"),
            ComponentType::Buffer => attr!("shape","invhouse"),
            ComponentType::Launch => attr!("shape","box"),
        }
    }

    fn node_id(component_type: &ComponentType, name: &str) -> NodeId {
        NodeId(Id::Plain(format!("{}_{}", component_type_to_letter(component_type), name)), None)
    }

    fn get_connection_set_name(possible_graph_connection_for_connection_set: Option<&GraphConnection>, component_type: &ComponentType, component_name: &str) -> String {

        fn taker(to_take: &Option<String>) -> String {
            match to_take {
                None => "".to_string(),
                Some(ref_str) => {
                    ref_str.to_owned()
                },
            }
        }

        if let Some(gc) = possible_graph_connection_for_connection_set {
            if (gc.src.1 == component_name) && (gc.src.0 == component_type) {
                return taker(gc.src.2)
            }
            return taker(gc.dst.2)
        }
        "".to_string()
    }

    let graph_components: HashMap<String, Vec<Node>> = components.iter().fold(
        HashMap::new(),
        |mut acc, (component_type, component_names)| {
            for component_name in component_names {
                let label = component_name;
                let connection_set_graph_connection: Option<&GraphConnection> = connections.iter().find(|connection| {
                    (
                        (connection.src.1 == *component_name) && (connection.src.0 == *component_type)
                    ) || (
                        (connection.dst.1 == *component_name) && (connection.dst.0 == *component_type)
                    )
                });

                acc.entry(get_connection_set_name(connection_set_graph_connection, component_type, component_name)).or_insert(vec![]).push(
                    Node::new(
                        node_id(component_type, component_name),
                        vec![attr!("label", label), attr!("style", "filled"), get_node_shape(component_type)]
                    )
                );
            }
            acc
        }
    );

    let mut stmts: Vec<Stmt> = graph_components.into_iter().fold(vec![], |mut acc, (connection_set, nodes)| {
        let title = format!("\"{}:\"", connection_set);
        let mut stmts = vec![Stmt::Attribute(attr!("label", title))];
        let mut stmts_to_add: Vec<Stmt> = nodes.into_iter().map(|gc| Stmt::Node(gc)).collect();
        if connection_set == "" {
            acc.append(&mut stmts_to_add);
            return acc
        }
        stmts.append(&mut stmts_to_add);
        acc.push(Stmt::Subgraph(Subgraph {
            id: id!(format!("cluster_nodes_{}", connection_set)),
            stmts
        }));
        acc
    });

    let mut edgs: Vec<Stmt> = connections.iter().map(|gc| {
        let mut lbl = format!("{:?}", gc.via);
        if gc.via == &OutputPort::Out {
            lbl = "\"\"".to_string()
        }
        Stmt::Edge(Edge {
            ty: EdgeTy::Pair(Vertex::N(node_id(&gc.src.0, &gc.src.1)), Vertex::N(node_id(&gc.dst.0, &gc.dst.1))),
            attributes: vec![attr!("label", lbl)]
        })
    }).collect();

    stmts.append(&mut edgs);

    Subgraph {
        id: id!("diagram"),
        stmts
    }

}

pub fn get_legend() -> Subgraph {
    subgraph!(
        "cluster_legend";
        attr!("color","black"),
        attr!("label","Legend"),
        subgraph!(
            "cluster_legend_launch";attr!("label","launch"),
            node!("legend_launch";attr!("label","\"\""),attr!("shape","box"),attr!("width","0.3"),attr!("style","filled"),attr!("height","0.3"))
        ),
        subgraph!(
            "cluster_legend_buffer";attr!("label","buffer"),
            node!("legend_buffer";attr!("label","\"\""),attr!("shape","invhouse"),attr!("width","0.3"),attr!("style","filled"),attr!("height","0.3"))
        ),
        subgraph!(
            "cluster_legend_junction";attr!("label","junction"),
            node!("legend_junction";attr!("label","\"\""),attr!("shape","oval"),attr!("width","0.3"),attr!("style","filled"),attr!("height","0.3"))
        ),
        subgraph!(
            "cluster_legend_faucet";attr!("label","faucet"),
            node!("legend_faucet";attr!("label","\"\""),attr!("shape","trapezium"),attr!("width","0.3"),attr!("style","filled"),attr!("height","0.3"))
        ),
        subgraph!(
            "cluster_legend_drain";attr!("label","drain"),
            node!("legend_drain";attr!("label","\"\""),attr!("shape","invtrapezium"),attr!("width","0.3"),attr!("style","filled"),attr!("height","0.3"))
        )
    )
}
