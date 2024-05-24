use actix_web::{web, HttpResponse, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

#[utoipa::path(
    get,
    path = "/pessoas",
    responses(
        (status = 200, description = "List all pessoas", body = [Pessoa])
    )
)]
pub async fn get_pessoas(pool: web::Data<MssqlPool>) -> impl Responder {
    // Implementation...
}

#[utoipa::path(
    get,
    path = "/pessoas/{id}",
    responses(
        (status = 200, description = "Get pessoa by ID", body = Pessoa),
        (status = 404, description = "Pessoa not found")
    ),
    params(
        ("id" = i32, Path, description = "Pessoa ID")
    )
)]
pub async fn get_pessoa_by_id(pool: web::Data<MssqlPool>, pessoa_id: web::Path<i32>) -> impl Responder {
    // Implementation...
}

#[utoipa::path(
    post,
    path = "/pessoas",
    request_body(content = Pessoa),
    responses(
        (status = 201, description = "Create new pessoa"),
        (status = 400, description = "Invalid input")
    )
)]
pub async fn create_pessoa(pool: web::Data<MssqlPool>, pessoa: web::Json<Pessoa>) -> impl Responder {
    // Implementation...
}

#[utoipa::path(
    put,
    path = "/pessoas/{id}",
    request_body(content = Pessoa),
    responses(
        (status = 200, description = "Update existing pessoa"),
        (status = 404, description = "Pessoa not found"),
        (status = 400, description = "Invalid input")
    ),
    params(
        ("id" = i32, Path, description = "Pessoa ID")
    )
)]
pub async fn update_pessoa(pool: web::Data<MssqlPool>, pessoa_id: web::Path<i32>, pessoa: web::Json<Pessoa>) -> impl Responder {
    // Implementation...
}

#[utoipa::path(
    delete,
    path = "/pessoas/{id}",
    responses(
        (status = 200, description = "Delete existing pessoa"),
        (status = 404, description = "Pessoa not found")
    ),
    params(
        ("id" = i32, Path, description = "Pessoa ID")
    )
)]
pub async fn delete_pessoa(pool: web::Data<MssqlPool>, pessoa_id: web::Path<i32>) -> impl Responder {
    // Implementation...
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_pessoas,
        get_pessoa_by_id,
        create_pessoa,
        update_pessoa,
        delete_pessoa
    ),
    components(schemas(Pessoa))
)]
pub struct ApiDoc;

pub fn swagger() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi())
}
