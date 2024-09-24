mod connections;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use connections::connection_keeper;

pub fn run() {
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let connection_keeper = Arc::new(
        Mutex::new(
            connection_keeper::ConnectionKeeper::new()
        )
    );

    connections::start_catching(addr, connection_keeper);
}