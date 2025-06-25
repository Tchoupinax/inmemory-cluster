use std::{
    io::{Read, Write},
    net::TcpStream,
};

use k8s_openapi::serde_json;

use crate::server::http::ui::get_stats::Stats;

pub fn request_stats_from_peer(addr: &str) -> Option<Stats> {
    let mut stream = TcpStream::connect(addr).ok()?;
    let _ = stream.write(format!("STATS|").as_bytes());

    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).ok()?;
    let response = &buffer[..size];

    serde_json::from_slice(response).ok()
}
