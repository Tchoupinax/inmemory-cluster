use actix_cors::Cors;
use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware, web, App, Error,
};

use crate::{SharedInternalDatabase, SharedPeers, SharedTimingStats, State};

pub fn create_http_app(
    peers: SharedPeers,
    internal_database: SharedInternalDatabase,
    time_stats: SharedTimingStats,
    state: State,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let _ = time_stats;
    App::new()
        .configure(super::router::create_router)
        //
        .app_data(web::Data::new(peers))
        .app_data(web::Data::new(internal_database))
        .app_data(web::Data::new(time_stats))
        //
        .app_data(web::Data::new(state))
        //
        .wrap(Cors::permissive())
        .wrap(middleware::Logger::default())
}
