use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{
    server::tcp::{commands::request_stats_from_peer, responses::stats::calculate_memory_usage},
    SharedInternalDatabase, SharedPeers, State,
};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub memory_mb: f64,
    pub key_count: usize,
}

#[derive(Serialize, Deserialize)]
struct PeerStats {
    peer_id: String,
    address: String,
    stats: Option<Stats>,
}

#[get("/stats")]
pub async fn get_stats_dn(
    peers: web::Data<SharedPeers>,
    internal_database: web::Data<SharedInternalDatabase>,
    state: web::Data<State>,
) -> impl Responder {
    let peers = peers.lock().unwrap();
    let mut stats_list: Vec<PeerStats> = Vec::new();

    let db_guard: std::sync::MutexGuard<'_, std::collections::HashMap<String, String>> =
        internal_database.lock().unwrap();
    let size = db_guard.len();
    drop(db_guard);

    let toto = internal_database.as_ref().clone();

    let stats = Stats {
        memory_mb: calculate_memory_usage(toto),
        key_count: size,
    };

    stats_list.push(PeerStats {
        peer_id: state.hostname.clone(),
        address: state.reacheable_url.clone(),
        stats: Some(stats),
    });

    for (peer_id, addr) in peers.iter() {
        let stats = request_stats_from_peer(addr);
        stats_list.push(PeerStats {
            peer_id: peer_id.clone(),
            address: addr.clone(),
            stats,
        });
    }

    HttpResponse::Ok().json(stats_list)
}
