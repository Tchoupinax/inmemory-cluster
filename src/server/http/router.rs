use actix_web::{
    get,
    web::{self},
    HttpResponse,
};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Healthy!")
}

pub fn create_router(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(super::data::data_add);
    cfg.service(super::data::data_get);
    cfg.service(super::ui::get_home);
    cfg.service(super::ui::get_stats_dn);
    cfg.service(super::ui::peers_table);
    cfg.service(super::ui::values_table);
}
