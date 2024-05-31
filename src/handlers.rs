use actix_web::{web, HttpResponse};
use sqlx::Pool;
use sqlx::Mssql;
use uuid::Uuid;

use crate::models::Pessoa;

pub async fn get_pessoas(pool: web::Data<Pool<Mssql>>) -> HttpResponse {
    let pessoas = sqlx::query_as!(Pessoa, "SELECT id, nome, email FROM pessoas")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(pessoas)
}

pub async fn get_pessoa(pool: web::Data<Pool<Mssql>>, pessoa_id: web::Path<Uuid>) -> HttpResponse {
    let id = pessoa_id.into_inner();
    let pessoa = sqlx::query_as!(Pessoa, "SELECT id, nome, email FROM pessoas WHERE id = ?", id)
        .fetch_one(pool.get_ref())
        .await;

    match pessoa {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn create_pessoa(pool: web::Data<Pool<Mssql>>, pessoa: web::Json<Pessoa>) -> HttpResponse {
    let new_pessoa = Pessoa {
        id: Uuid::new_v4(),
        nome: pessoa.nome.clone(),
        email: pessoa.email.clone(),
    };

    let result = sqlx::query!(
        "INSERT INTO pessoas (id, nome, email) VALUES (?, ?, ?)",
        new_pessoa.id,
        new_pessoa.nome,
        new_pessoa.email
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(new_pessoa),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_pessoa(
    pool: web::Data<Pool<Mssql>>,
    pessoa_id: web::Path<Uuid>,
    pessoa: web::Json<Pessoa>,
) -> HttpResponse {
    let id = pessoa_id.into_inner();
    let result = sqlx::query!(
        "UPDATE pessoas SET nome = ?, email = ? WHERE id = ?",
        pessoa.nome,
        pessoa.email,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(pessoa.into_inner()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_pessoa(pool: web::Data<Pool<Mssql>>, pessoa_id: web::Path<Uuid>) -> HttpResponse {
    let id = pessoa_id.into_inner();
    let result = sqlx::query!("DELETE FROM pessoas WHERE id = ?", id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
