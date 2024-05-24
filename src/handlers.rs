use actix_web::{web, HttpResponse, Responder};
use sqlx::MssqlPool;
use crate::models::Pessoa;

pub async fn get_pessoas(pool: web::Data<MssqlPool>) -> impl Responder {
    let pessoas = sqlx::query_as!(Pessoa, "SELECT id, nome, email FROM pessoas")
        .fetch_all(pool.get_ref())
        .await
        .expect("Failed to fetch pessoas");
    HttpResponse::Ok().json(pessoas)
}

pub async fn get_pessoa_by_id(pool: web::Data<MssqlPool>, pessoa_id: web::Path<i32>) -> impl Responder {
    let pessoa = sqlx::query_as!(Pessoa, "SELECT id, nome, email FROM pessoas WHERE id = ?", pessoa_id.into_inner())
        .fetch_one(pool.get_ref())
        .await
        .expect("Failed to fetch pessoa");
    HttpResponse::Ok().json(pessoa)
}

pub async fn create_pessoa(pool: web::Data<MssqlPool>, pessoa: web::Json<Pessoa>) -> impl Responder {
    sqlx::query!("INSERT INTO pessoas (nome, email) VALUES (?, ?)", pessoa.nome, pessoa.email)
        .execute(pool.get_ref())
        .await
        .expect("Failed to insert pessoa");
    HttpResponse::Created().finish()
}

pub async fn update_pessoa(pool: web::Data<MssqlPool>, pessoa_id: web::Path<i32>, pessoa: web::Json<Pessoa>) -> impl Responder {
    sqlx::query!("UPDATE pessoas SET nome = ?, email = ? WHERE id = ?", pessoa.nome, pessoa.email, pessoa_id.into_inner())
        .execute(pool.get_ref())
        .await
        .expect("Failed to update pessoa");
    HttpResponse::Ok().finish()
}

pub async fn delete_pessoa(pool: web::Data<MssqlPool>, pessoa_id: web::Path<i32>) -> impl Responder {
    sqlx::query!("DELETE FROM pessoas WHERE id = ?", pessoa_id.into_inner())
        .execute(pool.get_ref())
        .await
        .expect("Failed to delete pessoa");
    HttpResponse::Ok().finish()
}
