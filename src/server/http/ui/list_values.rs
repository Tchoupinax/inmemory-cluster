use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

use crate::SharedInternalDatabase;

#[get("/values")]
pub async fn values_table(values: web::Data<SharedInternalDatabase>) -> impl Responder {
    let values = values.lock().unwrap();

    let value_rows: String = values.iter()
        .map(|(k, v)| format!(
            r#"<tr class="border-b"><td class="p-2 font-mono">{}</td><td class="p-2 font-mono text-gray-600">{}</td></tr>"#,
            k, v
        ))
        .collect();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(value_rows)
}
