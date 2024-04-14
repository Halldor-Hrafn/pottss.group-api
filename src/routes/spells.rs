extern crate dotenv;

use dotenv::dotenv;
use std::env;

use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use sqlx::{postgres::PgConnection, Connection};

use sqlx::types::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Spell {
    pub id: Uuid,
    #[sqlx(rename = "created_at")]
    pub created_at: NaiveDateTime,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellRequest {
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

#[get("/spells/{id}")]
async fn get_spell(path: web::Path<Uuid>) -> impl Responder {
    dotenv().ok();

    let id = path.into_inner();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let spell = sqlx::query_as::<_, Spell>("Select * FROM public.spells WHERE id = $1")
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(spell)
}

#[post("/spells")]
async fn create_spell(spell: web::Json<SpellRequest>) -> impl Responder {
    dotenv().ok();

    let spell = spell.into_inner();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let spell = sqlx::query_as::<_, Spell>("INSERT INTO public.spells (name, description, rank, range, area, duration, actions, components, defense) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *")
        .bind(spell.name)
        .bind(spell.description)
        .bind(spell.rank)
        .bind(spell.range)
        .bind(spell.area)
        .bind(spell.duration)
        .bind(spell.actions)
        .bind(spell.components)
        .bind(spell.defense)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    println!("{:?}", spell);

    HttpResponse::Ok().json(spell)
}

#[put("/spells/{id}")]
async fn update_spell(path: web::Path<Uuid>, spell: web::Json<Spell>) -> impl Responder {
    dotenv().ok();

    let id = path.into_inner();
    let spell = spell.into_inner();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let spell = sqlx::query_as::<_, Spell>("UPDATE public.spells SET name = $1, description = $2, rank = $3, range = $4, area = $5, duration = $6, actions = $7, components = $8, defense = $9 WHERE id = $10 RETURNING *")
        .bind(spell.name)
        .bind(spell.description)
        .bind(spell.rank)
        .bind(spell.range)
        .bind(spell.area)
        .bind(spell.duration)
        .bind(spell.actions)
        .bind(spell.components)
        .bind(spell.defense)
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(spell)
}

#[delete("/spells/{id}")]
async fn delete_spell(path: web::Path<Uuid>) -> impl Responder {
    dotenv().ok();

    let id = path.into_inner();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let spell = sqlx::query_as::<_, Spell>("DELETE FROM public.spells WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(spell)
}

pub fn spell_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_spells)
        .service(create_spell)
        .service(update_spell)
        .service(delete_spell);
}