use rocket_db_pools::{sqlx, Database};

pub type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("sqlx")]
pub struct Db(sqlx::PgPool);
