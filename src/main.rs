extern crate dotenv;

use dotenv::dotenv;

pub mod routes;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Test {
    name: String,
    age: u8,
}

#[get("/test/{name}/{age}")]
async fn test(info: web::Path<Test>) -> impl Responder {
    let res = format!("Name: {}, Age: {}", info.name, info.age);

    HttpResponse::Ok().body(res)
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| App::new()
        .service(index)
        .service(test)
        .service(web::scope("/api/v1").configure(routes::spells::spell_config))
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
