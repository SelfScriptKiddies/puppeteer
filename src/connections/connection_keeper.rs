use log::trace;
use crate::connections::zombie_processor::Zombie;

#[derive(Debug)]
pub struct ConnectionKeeper {
    available_connections: Vec<Zombie>
}

impl ConnectionKeeper {
    pub fn new() -> ConnectionKeeper {
        ConnectionKeeper {
            available_connections: Vec::new()
        }
    }

    pub fn add_zombie(&mut self, zombie: Zombie) {
        trace!("{zombie:?} was inserted in global array");
        self.available_connections.push(zombie);
    }
}