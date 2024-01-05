use rocket::fairing::AdHoc;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};

use crate::models::db::Db;
use crate::models::res::ServicesConnectionsRes;
use crate::repositories::catalog_teo::get_services_db;

#[get("/services")]
pub async fn get_services(
    db: Connection<Db>,
) -> Result<Json<Vec<ServicesConnectionsRes>>, BadRequest<&'static str>> {
    match get_services_db(db).await {
        Ok(services) => Ok(Json(services)),
        Err(_) => Err(BadRequest("error from server")),
    }
}

pub fn stage_catalog() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .mount("/catalog", routes![get_services])
    })
}
