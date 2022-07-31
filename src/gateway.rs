use default_net;

fn get_interface(if_name: &str) -> default_net::interface::Interface {
    let interfaces = default_net::get_interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface: &default_net::Interface| iface.name == if_name)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", if_name));
    return interface;
}

pub fn get_gateway_mac_octets(if_name: &str) -> [u8; 6] {
    let interface = get_interface(if_name);
    if let Some(gateway) = interface.gateway {
        return gateway.mac_addr.octets();
    } else {
        panic!("No gateway found for interface: {}", if_name);
    }
}
