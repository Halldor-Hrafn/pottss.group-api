extern crate dotenv;

use dotenv::dotenv;
use std::env;

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use sqlx::{postgres::PgConnection, Connection};

use sqlx::types::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Spell {
    pub id: Uuid,
    // #[sqlx(rename = "created_at")]
    // pub created_at: NaiveDateTime,
    pub name: String,
    pub description: String,
    pub rank: i16,
    pub range: Option<String>,
    pub area: Option<String>,
    pub duration: Option<String>,
    pub actions: i16,
    pub components: Option<String>,
    pub defense: Option<String>,
}

#[get("/spells")]
async fn get_spells() -> impl Responder {
    dotenv().ok();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let spell = sqlx::query_as::<_, Spell>("Select * FROM public.spells WHERE name = $1")
        .bind("Approximate")
        .fetch_one(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(spell)
}

#[post("/spells")]
async fn create_spell(spell: web::Json<Spell>) -> impl Responder {
    HttpResponse::Ok().json(spell.0)
}

pub fn spell_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_spells)
        .service(create_spell);
}