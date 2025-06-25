use log::{debug, warn};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use crate::{server::tcp::add::send_command_add_to_all_peers, SharedInternalDatabase};
use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};

type SharedPeers = Arc<Mutex<BTreeMap<String, String>>>;

#[derive(Deserialize)]
struct MyPayload {
    #[serde(rename = "key")]
    key: String,
    #[serde(rename = "value")]
    value: String,
}

#[post("/data")]
async fn data_add(
    internal_database: web::Data<SharedInternalDatabase>,
    peers: web::Data<SharedPeers>,
    body: web::Json<MyPayload>,
) -> impl Responder {
    let peers = peers.get_ref().clone();
    let internal_database = internal_database.get_ref().clone();

    match internal_database.lock() {
        Ok(mut data) => {
            data.insert(body.key.clone(), body.value.clone());
        }
        Err(poisoned) => {
            warn!("Mutex poisoned, recovering: {:?}", poisoned);
        }
    };

    debug!("Key {} added in my database", body.key.clone());

    send_command_add_to_all_peers(peers, body.key.clone(), body.value.clone());

    HttpResponse::Ok().body("OK")
}
