use std::net::SocketAddr;

pub mod connections;
mod initializer;

fn main() {
    initializer::init();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8000));
    connections::start_catching(addr);
}
