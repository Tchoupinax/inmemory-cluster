use std::{
    collections::BTreeMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Utc};
use log::error;
use speculate::speculate;

pub fn send_command_add_to_all_peers(
    peers: Arc<Mutex<BTreeMap<String, String>>>,
    key: String,
    value: String,
) {
    let command = generate_command_add(key, value, Utc::now());

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

fn generate_command_add(key: String, value: String, date: DateTime<Utc>) -> String {
    format!("ADD|key={},value={},date={}", key, value, date.to_rfc3339())
}

extern crate speculate as other_speculate;
speculate! {
    describe "when calling with one key and one value" {
        it "should return the correct formatted ADD command" {
            let date = Utc::now();

            assert_eq!(
                generate_command_add("a".to_string(), "b".to_string(), date),
                format!("ADD|key=a,value=b,date={}", date.to_rfc3339())
            );
        }
    }
}
