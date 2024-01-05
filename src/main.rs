#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

mod models;
mod repositories;
mod routes;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/catalog", routes::catalog_teo::get_services()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(routes::catalog_teo::stage_catalog())
}
