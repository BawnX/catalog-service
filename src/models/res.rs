use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ServicesConnectionsRes {
    pub id: i64,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    pub type_name: String,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}
