use clap::Parser;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use psutil::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// ports to find
    #[clap(value_delimiter = ',', index = 1)]
    port: Vec<u16>,
}

fn main() {
    let args = Args::parse();
    println!("finding port {:?}!", args.port);

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                if args.port.contains(&tcp_si.local_port) {
                    si.associated_pids.iter().for_each(|pid| {
                        print_and_kill(*pid);
                    });
                }
            }
            ProtocolSocketInfo::Udp(udp_si) => {
                if args.port.contains(&udp_si.local_port) {
                    si.associated_pids.iter().for_each(|pid| {
                        print_and_kill(*pid);
                    });
                }
            }
        }
    }
}

fn print_and_kill(pid: u32) {
    let p = process::Process::new(pid).unwrap();
    println!("killing {}, {}", p.pid(), p.name().unwrap());
    p.kill().unwrap();
}
