use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::server::tcp::commands::copy::send_command_copy_to_peer;
use crate::server::tcp::expose_known_peers;
use crate::SharedInternalDatabase;

use log::{debug, warn};

pub fn identification_answer(
    peers: Arc<Mutex<BTreeMap<String, String>>>,
    database: SharedInternalDatabase,
    parts: Vec<&str>,
    my_name: String,
    my_address: String,
) -> Vec<u8> {
    debug!("Identification answer");

    let properties: Vec<&str> = parts[1].split(",").collect();

    let name: Vec<&str> = properties[0].split("=").collect();
    let addr: Vec<&str> = properties[1].split("=").collect();

    match peers.lock() {
        Ok(mut data) => data.insert(name[1].to_string(), addr[1].to_string()),
        Err(poisoned) => {
            warn!("Mutex poisoned, recovering: {:?}", poisoned);
            let mut data = poisoned.into_inner();
            data.insert(name[1].to_string(), addr[1].to_string())
        }
    };

    let addr = addr[1].to_string();
    thread::spawn(move || {
        send_command_copy_to_peer(addr, database);
    });

    expose_known_peers(Arc::clone(&peers), my_name.clone(), my_address.clone())
}
