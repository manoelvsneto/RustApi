use actix_web::web;
use paperclip::actix::web::{self, Json};
use paperclip::actix::Apiv2Schema;

use crate::handlers::{create_pessoa, delete_pessoa, get_pessoa, get_pessoas, update_pessoa};
use crate::models::Pessoa;

#[derive(Apiv2Schema)]
struct CreatePessoaBody {
    nome: String,
    email: String,
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/pessoas", web::get().to(get_pessoas))
            .route("/pessoas/{id}", web::get().to(get_pessoa))
            .route("/pessoas", web::post().to(create_pessoa))
            .route("/pessoas/{id}", web::put().to(update_pessoa))
            .route("/pessoas/{id}", web::delete().to(delete_pessoa)),
    );
}
