use actix_web::{http::header::ContentType, test};
use inmemory_cluster::server::http::create_http_app;
use inmemory_cluster::timing::TimingStats;
use inmemory_cluster::{SharedInternalDatabase, SharedPeers, SharedTimingStats, State};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

#[actix_web::test]
async fn test_post_data_add() {
    let peers: SharedPeers = Arc::new(Mutex::new(BTreeMap::new()));
    let internal_database: SharedInternalDatabase = Arc::new(Mutex::new(HashMap::new()));
    let timing: SharedTimingStats = Arc::new(Mutex::new(TimingStats::new()));

    peers
        .lock()
        .unwrap()
        .insert("peer1".into(), "1.2.3.4".into());

    let app = test::init_service(create_http_app(
        peers,
        internal_database,
        timing,
        State {
            hostname: "".to_string(),
            reacheable_url: "".to_string(),
        },
    ))
    .await;

    let req = test::TestRequest::post()
        .uri("/data")
        .insert_header(ContentType::json())
        .set_payload(r#"{"key":"my-key", "value": "my-value"}"#)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let response_str = std::str::from_utf8(&body).unwrap();
    assert!(response_str.contains("OK"));
}
