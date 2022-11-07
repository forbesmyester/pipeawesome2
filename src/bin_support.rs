use std::collections::HashMap;
use crate::{waiter::WaiterError, config::ComponentType, motion::Journey};

pub fn waiter_error_to_string(waiter_err: WaiterError, id_to_component_type_name: &HashMap<usize, (ComponentType, String)>) -> String {

    fn component_type_name_to_string(ct: &ComponentType, n: &str) -> String {
        format!("{}:{}", ct, n)
    }

    let waiter_src: Option<(&ComponentType, &String)> = waiter_err.caused_by_error_source();
    let motion_src: Option<&(ComponentType, String)> = match waiter_err.caused_by_error().map(|err| err.journey_source()).flatten() {
        Some(journey_source) => id_to_component_type_name.get(journey_source),
        _ => None
    };
    let motion_dst: Option<&(ComponentType, String)> = match waiter_err.caused_by_error().map(|err| err.journey()).flatten() {
        Some(Journey { dst, .. }) => id_to_component_type_name.get(&dst),
        _ => None
    };

    let (src, dst) = match (waiter_src, motion_src, motion_dst) {
        (_, Some(motion_src), Some(motion_dst)) => (component_type_name_to_string(&motion_src.0, &motion_src.1), component_type_name_to_string(&motion_dst.0, &motion_dst.1)),
        (_, Some(motion_src), _) => (component_type_name_to_string(&motion_src.0, &motion_src.1), "Unknown Destination".to_string()),
        (Some(waiter_src), _, _) => (component_type_name_to_string(waiter_src.0, waiter_src.1), "Unknown Destination".to_string()),
        _ => ("Unknown Source".to_string(), "Unknown Destination".to_string()),
    };

    format!("{} | {} - {:?}", src, dst, waiter_err.description())
}

