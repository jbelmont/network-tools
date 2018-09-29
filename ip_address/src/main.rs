extern crate pnet;

use std::env;

use pnet::datalink;
use pnet::datalink::NetworkInterface;

fn main() {
    let interface_name = env::args().nth(1).unwrap();
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;
    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                            .filter(interface_names_match)
                            .next()
                            .unwrap();
    for i_face in interface.ips {
        if i_face.prefix() == 23 {
            println!("{}", i_face.ip().to_string());
        }
    }
}
