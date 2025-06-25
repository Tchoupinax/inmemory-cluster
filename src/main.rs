mod env;
mod logger;
mod server;
mod sigterm;
mod timing;

use actix_web::HttpServer;
use gethostname::gethostname;
use inmemory_cluster::State;
use log::debug;
use log::error;
use log::info;
use rand::Rng;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self};

use crate::env::Environment;
use crate::timing::TimingStats;

pub type SharedPeers = Arc<Mutex<BTreeMap<String, String>>>;
pub type SharedInternalDatabase = Arc<Mutex<HashMap<String, String>>>;
pub type SharedTimingStats = Arc<Mutex<TimingStats>>;

#[tokio::main]
async fn main() {
    logger::init_logger();

    let peers: SharedPeers = Arc::new(Mutex::new(BTreeMap::<String, String>::new()));
    let internal_database: SharedInternalDatabase =
        Arc::new(Mutex::new(HashMap::<String, String>::new()));

    // Somewhere in app state setup
    let timing_stats: SharedTimingStats = Arc::new(Mutex::new(TimingStats::new()));

    let config_environment: Environment = match std::env::var("ENV") {
        Ok(x) => match x.parse() {
            Ok(env) => env,
            Err(_) => Environment::Dev,
        },
        Err(_) => Environment::Dev,
    };

    let config_hostname = match std::env::var("HOSTNAME") {
        Ok(hostname) => hostname,
        Err(_) => match config_environment {
            Environment::Prod => format!("{:?}", gethostname()),
            Environment::Dev => format!("hostname-{}", process::id().to_string()),
        },
    };

    const CONFIG_NAMESPACE: &str = "inmemory-cluster";

    let random_port: u16 = rand::thread_rng().gen_range(8100..8150);
    let config_port_http: u16 = match std::env::var("PORT_HTTP") {
        Ok(x) => match x.parse() {
            Ok(x) => x,
            Err(_) => random_port,
        },
        Err(_) => random_port,
    };

    info!("🚀 Welcome to inmemory-cluster");

    let ctrlc_handle =
        sigterm::sigterm_handler(peers.clone(), config_hostname.clone(), config_environment);

    let (listener, port) = server::tcp::tcp::start_listen();

    let config_my_own_url = match config_environment {
        Environment::Prod => format!("{}.{}", config_hostname.to_string(), CONFIG_NAMESPACE),
        Environment::Dev => format!("{}:{}", "0.0.0.0".to_string(), port.to_string()),
    };

    let peers_clone = Arc::clone(&peers);
    server::tcp::warn_peers_you_exist(
        config_environment,
        peers_clone,
        config_my_own_url.clone(),
        port,
        config_hostname.clone(),
    )
    .await;

    let listener_copy = listener.try_clone().unwrap();
    let peers_clone = Arc::clone(&peers);
    let internal_database_clone = Arc::clone(&internal_database);
    let timing_stats_clone = Arc::clone(&timing_stats);

    let name = config_hostname.clone();
    let url = config_my_own_url.clone();

    thread::spawn(move || {
        for stream in listener_copy.incoming() {
            match stream {
                Ok(stream) => {
                    debug!(
                        "New connection opened from: {}",
                        stream.peer_addr().unwrap()
                    );

                    let db = Arc::clone(&internal_database_clone);
                    let peers_clone2 = Arc::clone(&peers_clone);

                    let timing_stats_clone = Arc::clone(&timing_stats_clone);

                    let name_clone = name.clone();
                    let url_clone = url.clone();

                    thread::spawn(move || {
                        server::tcp::handle_client(
                            stream,
                            db,
                            peers_clone2,
                            name_clone,
                            url_clone,
                            timing_stats_clone,
                        )
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    });

    let state = State {
        hostname: config_hostname.clone().to_string(),
        reacheable_url: config_my_own_url.clone().to_string(),
    };

    let _ = match HttpServer::new(move || {
        let peers_clone = Arc::clone(&peers);
        let internal_database_clone = Arc::clone(&internal_database);
        let timing_stats_clone = Arc::clone(&timing_stats);

        server::http::create_http_app(
            peers_clone,
            internal_database_clone,
            timing_stats_clone,
            state.clone(),
        )
    })
    .workers(2)
    .bind(("0.0.0.0", config_port_http))
    {
        Ok(server) => server.run(),
        Err(err) => {
            error!("{}", err);
            panic!("Failed to start");
        }
    }
    .await;

    drop(listener);

    // Wait for the handler thread before exiting (if needed)
    ctrlc_handle.join().unwrap();
}
