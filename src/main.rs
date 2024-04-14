extern crate dotenv;

use dotenv::dotenv;

pub mod routes;

use actix_web::{web, App, HttpServer};

pub fn api_v1_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(routes::spells::spell_config)
            .configure(routes::traits::trait_config)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| App::new()
        .configure(api_v1_config)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
