use rocket::fairing::AdHoc;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};
use crate::models;

use crate::models::db::Db;
use crate::models::res::{ResultRes, ServicesConnectionsRes};
use crate::repositories::catalog_teo::{get_services_db, create_services_db};

#[get("/services")]
pub async fn get_services(
    db: Connection<Db>,
) -> Result<Json<Vec<ServicesConnectionsRes>>, BadRequest<&'static str>> {
    match get_services_db(db).await {
        Ok(services) => Ok(Json(services)),
        Err(_) => Err(BadRequest("error from server")),
    }
}

#[post("/services", format = "application/json", data = "<data>")]
pub async fn create_services(
    db: Connection<Db>,
    data:  Json<models::req::CreateService>
) -> Result<Json<ResultRes<bool>>, BadRequest<Json<ResultRes<bool>>>>{
    match create_services_db(db, data.source.clone(), data.target.clone(), data.type_connection.clone()).await {
        Ok(is_correct) => Ok(Json(ResultRes{data: { is_correct }})),
        Err(is_correct) => Err(BadRequest(Json(ResultRes{data: { is_correct }}))),
    }
}


pub fn stage_catalog() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .mount("/catalog", routes![get_services])
            .mount("/catalog", routes![create_services])
    })
}
