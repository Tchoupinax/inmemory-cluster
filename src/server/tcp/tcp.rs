use log::info;
use rand::Rng;
use std::{env, net::TcpListener};

pub fn start_listen() -> (TcpListener, u16) {
    let random_port: u16 = rand::thread_rng().gen_range(8000..8050);
    let port = env::var("PORT_TCP").unwrap_or_else(|_| random_port.to_string());

    let port_num: u16 = port.parse().expect("PORT_TCP must be a valid number");

    let socket_string = format!("0.0.0.0:{}", port_num);

    if let Ok(listener) = TcpListener::bind(&socket_string) {
        info!("TCP server is listening on {}", socket_string);
        return (listener, port_num);
    }

    panic!("Failed to bind to a port in range 8000-8050");
}
