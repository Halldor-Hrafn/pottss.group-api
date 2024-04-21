extern crate dotenv;

use dotenv::dotenv;
use std::env;

use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use sqlx::{postgres::PgConnection, Connection};

use sqlx::types::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Trait {
    pub id: Uuid,
    #[sqlx(rename = "created_at")]
    pub created_at: NaiveDateTime,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitRequest {
    pub name: String,
    pub description: String,
    _type: String,
}

async fn insert_spells_trait(conn: &mut PgConnection, spell_id: Uuid, trait_id: Uuid) {
    sqlx::query("INSERT INTO public.spells_traits (spell_id, trait_id) VALUES ($1, $2)")
        .bind(spell_id)
        .bind(trait_id)
        .execute(conn)
        .await
        .unwrap();
}

async fn insert_equipment_trait(conn: &mut PgConnection, equipment_id: Uuid, trait_id: Uuid) {
    sqlx::query("INSERT INTO public.equipment_traits (equipment_id, trait_id) VALUES ($1, $2)")
        .bind(equipment_id)
        .bind(trait_id)
        .execute(conn)
        .await
        .unwrap();
}

#[get("/traits")]
async fn get_traits() -> impl Responder {
    dotenv().ok();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let _trait = sqlx::query_as::<_, Trait>("Select * FROM public.traits")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(_trait)
}

#[get("/traits/{id}")]
async fn get_trait(path: web::Path<Uuid>) -> impl Responder {
    dotenv().ok();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let _trait = sqlx::query_as::<_, Trait>("Select * FROM public.traits WHERE id = $1")
        .bind(path.into_inner())
        .fetch_one(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(_trait)
}

#[post("/traits")]
async fn create_trait(trait_request: web::Json<TraitRequest>) -> impl Responder {
    dotenv().ok();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let _trait = sqlx::query_as::<_, Trait>("INSERT INTO public.traits (name, description) VALUES ($1, $2) RETURNING *")
        .bind(&trait_request.name)
        .bind(&trait_request.description)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(_trait)
}

#[put("/traits/{id}")]
async fn update_trait(path: web::Path<Uuid>, trait_request: web::Json<Trait>) -> impl Responder {
    dotenv().ok();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let _trait = sqlx::query_as::<_, Trait>("UPDATE public.traits SET name = $1, description = $2 WHERE id = $3 RETURNING *")
        .bind(&trait_request.name)
        .bind(&trait_request.description)
        .bind(path.into_inner())
        .fetch_one(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(_trait)
}

#[delete("/traits/{id}")]
async fn delete_trait(path: web::Path<Uuid>) -> impl Responder {
    dotenv().ok();

    let database_string = env::var("DATABASE_CONNECTION_STRING")
        .expect("DATABASE_CONNECTION_STRING must be set");

    let mut conn = PgConnection::connect(&database_string)
        .await
        .expect("Failed to connect to database");

    let _trait = sqlx::query_as::<_, Trait>("DELETE FROM public.traits WHERE id = $1 RETURNING *")
        .bind(path.into_inner())
        .fetch_one(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().json(_trait)
}

pub fn trait_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_traits)
        .service(get_trait)
        .service(create_trait)
        .service(update_trait)
        .service(delete_trait);
}
