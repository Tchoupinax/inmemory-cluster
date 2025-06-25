use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use speculate::speculate;

pub fn expose_known_peers(
    peers: Arc<Mutex<BTreeMap<String, String>>>,
    my_name: String,
    my_address: String,
) -> Vec<u8> {
    let mut returned_peers = format!("ME|hostname={},addr={}", my_name, my_address);

    for (key, value) in peers.lock().unwrap().clone().into_iter() {
        if returned_peers.len() == 0 {
            returned_peers += &format!(
                "PEER|hostname={},addr={}",
                key.to_string(),
                value.to_string()
            );
        } else {
            returned_peers += &format!(
                "\nPEER|hostname={},addr={}",
                key.to_string(),
                value.to_string()
            );
        }
    }

    returned_peers.as_bytes().to_vec()
}

extern crate speculate as other_speculate;
speculate! {
    describe "when no peers are registered" {
        it "should return an empty string" {
            let peers = Arc::new(Mutex::new(BTreeMap::<String, String>::new()));
            assert_eq!(
                expose_known_peers(peers, "myName".to_string(), "address:8888".to_string()),
                "ME|hostname=myName,addr=address:8888".as_bytes().to_vec()
            );
        }
    }


    describe "when one peer is registered" {
        it "should return an empty string" {
            let peers = Arc::new(Mutex::new(BTreeMap::<String, String>::new()));
            peers.lock().unwrap().insert("1".to_string(), "11".to_string());

            assert_eq!(
                expose_known_peers(peers, "myName".to_string(), "address:8888".to_string()),
                "ME|hostname=myName,addr=address:8888\nPEER|hostname=1,addr=11".as_bytes().to_vec()
            );
        }
    }

    describe "when three peers are registered" {
        it "should return an empty string" {
            let peers = Arc::new(Mutex::new(std::collections::BTreeMap:: <String,String> ::new()));
            peers.lock().unwrap().insert("1".to_string(), "11".to_string());
            peers.lock().unwrap().insert("2".to_string(), "22".to_string());
            peers.lock().unwrap().insert("3".to_string(), "33".to_string());

            assert_eq!(
                expose_known_peers(peers, "myName".to_string(), "address:8888".to_string()),
                "ME|hostname=myName,addr=address:8888\nPEER|hostname=1,addr=11\nPEER|hostname=2,addr=22\nPEER|hostname=3,addr=33".as_bytes().to_vec()
            );
        }
    }
}
