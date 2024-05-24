use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Pessoa {
    pub id: Option<i32>,
    pub nome: String,
    pub email: String,
}
