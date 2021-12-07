use crate::config::{ Config, ComponentType, Connection };
use crate::connectable::OutputPort;
use std::collections::HashMap;
use std::collections::HashSet;

use graphviz_rust::dot_structures::*;
use graphviz_rust::dot_generator::*;
use graphviz_rust::{exec, parse};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::{PrinterContext,DotPrinter};
use graphviz_rust::attributes::*;

// pub fn get_graph_components(config: &Config) -> HashMap<ComponentType, HashSet<String>> {
// }

pub struct GraphConnection<'a> {
    src: (&'a ComponentType, &'a str),
    via: &'a OutputPort,
    dst: (&'a ComponentType, &'a str),
}

pub fn convert_connection_to_graph_connection<'a>(mut acc: Vec<GraphConnection<'a>>, connections: Vec<&'a Connection>) -> Vec<GraphConnection<'a>> {

    let mut src: Option<(&ComponentType, &str, Option<&OutputPort>)> = None;
    let mut dst: Option<(&ComponentType, &str, Option<&OutputPort>)> = None;
    let mut r: Vec<GraphConnection> = vec![];

    for connection in connections {
        dst = match connection {
            Connection::MiddleConnection { component_type, component_name, output_port, .. } => Some((component_type, component_name, Some(output_port))),
            Connection::StartConnection { component_type, component_name, output_port } => Some((component_type, component_name, Some(output_port))),
            Connection::EndConnection { component_type, component_name, .. } => Some((component_type, component_name, None)),
        };
        match (src, dst) {
            (Some((src_0, src_1, Some(src_2))), Some(dst)) => {
                r.push(GraphConnection {
                    src: (src_0, src_1),
                    via: src_2,
                    dst: (dst.0, dst.1),
                });
            },
            _ => (),
        }
        src = match dst {
            Some((dst_type, dst_name, Some(output_port))) => Some((dst_type, dst_name, Some(output_port))),
            _ => None,
        };
    }

    acc.append(&mut r);
    acc

}


pub fn convert_connection_components_fold<'a>(mut acc: HashMap<&'a ComponentType, HashSet<&'a str>>, connections: Vec<&'a Connection>) -> HashMap<&'a ComponentType, HashSet<&'a str>> {
    use std::iter::FromIterator;

    for connection in connections {
        let comp: (&ComponentType, &str) = match connection {
            Connection::MiddleConnection { component_type, component_name, .. } => (component_type, component_name),
            Connection::StartConnection { component_type, component_name, .. } => (component_type, component_name),
            Connection::EndConnection { component_type, component_name, .. } => (component_type, component_name),
        };

        acc.entry(comp.0)
            .and_modify(|x| {
                x.insert(comp.1);
            })
            .or_insert(HashSet::<&str>::from_iter([comp.1]));
    }


    acc
}

pub fn get_graph(subgraph: Vec<Subgraph>) -> String {

    let stmts = subgraph.into_iter().map(|sg| { Stmt::Subgraph(sg) }).collect();

    let graph = Graph::Graph {
        id: Id::Plain("g_get_graph".to_string()),
        strict: true,
        stmts
    };

    let mut ctx = PrinterContext::default();
    graph.print(&mut ctx)
}


pub fn get_diagram(components: HashMap<&ComponentType, HashSet<&str>>, connections: Vec<GraphConnection>) -> Subgraph {

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

    let graph_components: Vec<Node> = components.iter().fold(
        vec![],
        |mut acc, (component_type, component_names)| {
            let mut nodes: Vec<Node> = component_names.iter().map(
                |name| {
                    let label = name; // format!("{} {}", get_node_icon(component_type), name);
                    Node::new(
                        node_id(component_type, name),
                        vec![attr!("label", label), attr!("style", "filled"), get_node_shape(component_type)]
                    )
                }
            ).collect();
            acc.append(&mut nodes);
            acc
        }
    );

    let mut stmts: Vec<Stmt> = graph_components.into_iter().map(|gc| Stmt::Node(gc)).collect();

    let mut edgs: Vec<Stmt> = connections.iter().map(|gc| {
        let lbl = format!("{:?}", gc.via);
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
