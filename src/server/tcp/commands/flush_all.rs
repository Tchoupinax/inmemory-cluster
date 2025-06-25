use std::{
    collections::BTreeMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use log::error;
use speculate::speculate;

pub fn send_command_flush_all_to_all_peers(peers: Arc<Mutex<BTreeMap<String, String>>>) {
    let command = generate_command_flush_all();

    for peer in peers.clone().lock().unwrap().clone().into_iter() {
        let (_, addr) = peer;

        match TcpStream::connect(&addr) {
            Ok(mut stream) => {
                let _ = stream.write(command.as_bytes());
            }
            Err(x) => {
                error!("{}", x);
            }
        }
    }
}

fn generate_command_flush_all() -> String {
    "FLUSHALL|".to_string()
}

extern crate speculate as other_speculate;
speculate! {
    describe "when calling flush all command" {
        it "should return formated command" {
            assert_eq!(
                generate_command_flush_all(),
                "FLUSHALL|".to_string(),
            );
        }
    }
}
