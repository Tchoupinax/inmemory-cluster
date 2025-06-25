use std::{
    collections::BTreeMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use log::{error, info};

pub fn disconnect_from_peers(peers: &Arc<Mutex<BTreeMap<String, String>>>, hostname: String) {
    for peer in peers.clone().lock().unwrap().clone().into_iter() {
        let addr = peer.1;

        info!("Disconnect message sent to {}", addr);

        match TcpStream::connect(&addr) {
            Ok(mut stream) => {
                let _ = stream.write(format!("DISCONNECT|hostname={}", hostname).as_bytes());
            }
            Err(x) => {
                error!("{}", x);
            }
        }
    }
}
