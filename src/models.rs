use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Pessoa {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
}
