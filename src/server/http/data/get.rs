use crate::SharedInternalDatabase;
use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use serde::Serialize;

#[derive(Serialize)]
struct ValueResponse {
    value: Option<String>,
}

#[get("/data/{key}")]
async fn data_get(
    internal_database: web::Data<SharedInternalDatabase>,
    path: web::Path<String>,
) -> impl Responder {
    let internal_database = internal_database.get_ref().clone();

    let key = path.into_inner();

    let value = internal_database
        .lock()
        .ok()
        .and_then(|db| db.get(&key).cloned());

    let response = ValueResponse { value };

    HttpResponse::Ok().json(response)
}
