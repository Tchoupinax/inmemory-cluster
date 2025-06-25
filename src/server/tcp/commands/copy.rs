use std::{io::Write, net::TcpStream};

use log::{debug, error};
use speculate::speculate;

use crate::SharedInternalDatabase;

pub fn send_command_copy_to_peer(peer_addr: String, database: SharedInternalDatabase) {
    debug!("Sending command copy to peer {}", peer_addr);
    let command = generate_commmand_copy(database);

    if command.len() == 0 {
        debug!("Nothing to copy");
    } else {
        match TcpStream::connect(&peer_addr) {
            Ok(mut stream) => match stream.write(&command) {
                Ok(_) => {
                    debug!("Command has been sent");
                }
                Err(x) => {
                    error!("Error when sending COPY command: {}", x);
                }
            },
            Err(x) => {
                error!("Error enabling TCP connection: {}", x);
            }
        }
    }
}

fn generate_commmand_copy(database: SharedInternalDatabase) -> Vec<u8> {
    let mut copy_data = "".to_string();

    for (key, value) in database.clone().lock().unwrap().clone().into_iter() {
        if copy_data.len() == 0 {
            copy_data += &format!("COPY|k={},v={}", key.to_string(), value.to_string());
        } else {
            copy_data += &format!("\nCOPY|k={},v={}", key.to_string(), value.to_string());
        }
    }

    copy_data.as_bytes().to_vec()
}

extern crate speculate as other_speculate;
speculate! {
    describe "when calling with one key and one value" {
        it "should return the correct formatted ADD command" {
            let database =  std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap:: <String,String> ::new()));
            database.lock().unwrap().insert("toto".to_string(), "tata".to_string());
            database.lock().unwrap().insert("toto2".to_string(), "tata2".to_string());

            assert_eq!(
                String::from_utf8(generate_commmand_copy(database)).unwrap(),
                "COPY|k=toto,v=tata\nCOPY|k=toto2,v=tata2"
            );
        }
    }
}
