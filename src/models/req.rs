use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateService {
    pub source: String,
    pub target: String,
    pub type_connection: String
}
