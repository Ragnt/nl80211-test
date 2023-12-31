use nl80211_ng::{get_interface_info_name, interface, set_interface_monitor, Interface, Nl80211};
use rand::Rng;

fn main() {
    let mut socket: Nl80211 = nl80211_ng::Nl80211::new().unwrap();
    let ifaces = socket.get_interfaces();

    let first_iface: &Interface = ifaces.values().next().unwrap();

    let channel_map = first_iface.get_frequency_list_simple();

    println!(
        "first_iface: {:?} | {:?}",
        first_iface.name_as_string(),
        channel_map
    );
}
