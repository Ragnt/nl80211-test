use nl80211_ng::{get_interface_info_name, interface, set_interface_monitor, Interface, Nl80211};
use rand::Rng;

fn main() {
    let mut socket: Nl80211 = nl80211_ng::Nl80211::new().unwrap();
    let ifaces = socket.get_interfaces();

    for iface in ifaces.clone() {
        println!("{}\n{}", iface.0,iface.1.pretty_print());
        let _ = socket.set_interface_monitor(false, iface.1.index.unwrap());
    }
}
