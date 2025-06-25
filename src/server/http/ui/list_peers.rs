use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

use crate::{SharedPeers, State};

#[get("/peers")]
pub async fn peers_table(peers: web::Data<SharedPeers>, state: web::Data<State>) -> impl Responder {
    let peers = peers.lock().unwrap();

    let mut peer_rows = format!(
        r#"<tr class="border-b"><td class="p-2 font-mono">{}</td><td class="p-2 font-mono text-gray-600">{}</td></tr>"#,
        state.hostname, state.reacheable_url
    );

    peer_rows += &peers
    .iter()
    .map(|(k, v)| format!(
        r#"<tr class="border-b"><td class="p-2 font-mono">{}</td><td class="p-2 font-mono text-gray-600">{}</td></tr>"#,
        k, v
    ))
    .collect::<String>();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(peer_rows)
}
