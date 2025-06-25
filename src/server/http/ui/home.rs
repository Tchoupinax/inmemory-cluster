use actix_web::{get, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/")]
async fn get_home() -> impl Responder {
    let tera = match Tera::new("templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    println!(
        "Loaded templates: {:?}",
        tera.get_template_names().collect::<Vec<_>>()
    );

    let context = Context::new();

    let rendered = tera.render("home.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}
