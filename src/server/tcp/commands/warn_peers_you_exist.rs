use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client};
use log::debug;

use crate::env::Environment;

pub async fn warn_peers_you_exist(
    environment: Environment,
    peers: Arc<Mutex<BTreeMap<String, String>>>,
    my_own_url: String,
    my_port: u16,
    hostname: String,
) {
    debug!("Warn peers");

    match environment {
        Environment::Prod => {
            let client = match Client::try_default().await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            let namespace =
                std::env::var("NAMESPACE").unwrap_or_else(|_| "inmemory-cluster".to_string());

            let pods: Api<Pod> = Api::namespaced(client, &namespace);

            // List pods with default parameters
            let lp = ListParams::default();
            let pod_list = match pods.list(&lp).await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            for pod in pod_list {
                if let Some(addr) = pod.status.and_then(|s| s.pod_ip) {
                    let peers = peers.clone();
                    connect_to_peer(&addr, my_port, peers, my_own_url.clone(), hostname.clone())
                }
            }
        }
        Environment::Dev => {
            for index in 8000..8050 {
                if index != my_port {
                    let peers = peers.clone();
                    connect_to_peer(
                        "0.0.0.0",
                        index,
                        peers,
                        my_own_url.clone(),
                        hostname.clone(),
                    )
                }
            }
        }
    }
}

fn connect_to_peer(
    peer_ip: &str,
    peer_port: u16,
    peers: Arc<Mutex<BTreeMap<String, String>>>,
    my_own_url: String,
    hostname: String,
) {
    let addr = format!("{}:{}", peer_ip, peer_port);

    match TcpStream::connect(&addr) {
        Ok(mut stream) => {
            debug!("Peer found {}", addr);

            let _ = stream.write(
                format!("IDENTIFICATION|hostname={},addr={}", hostname, my_own_url).as_bytes(),
            );

            let peers = peers.clone();
            std::thread::spawn(move || {
                let peers = peers.clone();
                let mut buffer = [0; 512];
                while let Ok(n) = stream.read(&mut buffer) {
                    if n == 0 {
                        break;
                    }

                    let answer = String::from_utf8_lossy(&buffer[..n]);
                    debug!("Identification answer => {}", answer);

                    if answer.contains("|") {
                        let commands: Vec<&str> = answer.split("\n").collect();
                        for command in commands {
                            let parts: Vec<&str> = command.split("|").collect();

                            match parts[0] {
                                "PEER" => {
                                    let properties: Vec<&str> = parts[1].split(",").collect();
                                    let name =
                                        match properties.get(0).and_then(|s| s.split('=').nth(1)) {
                                            Some(p) => p.to_string(),
                                            None => "".to_string(),
                                        };
                                    let addr =
                                        match properties.get(1).and_then(|s| s.split('=').nth(1)) {
                                            Some(p) => p.to_string(),
                                            None => "".to_string(),
                                        };

                                    // Do not add myself
                                    if addr != my_own_url {
                                        peers.lock().unwrap().insert(name, addr);
                                    }
                                }
                                "ME" => {
                                    let properties: Vec<&str> = parts[1].split(",").collect();
                                    let name =
                                        match properties.get(0).and_then(|s| s.split('=').nth(1)) {
                                            Some(p) => p.to_string(),
                                            None => "".to_string(),
                                        };
                                    let addr =
                                        match properties.get(1).and_then(|s| s.split('=').nth(1)) {
                                            Some(p) => p.to_string(),
                                            None => "".to_string(),
                                        };

                                    peers.lock().unwrap().insert(name, addr);
                                }
                                _ => todo!("ok"),
                            };
                        }
                    }
                }
            });
        }
        Err(_) => {}
    }
}
