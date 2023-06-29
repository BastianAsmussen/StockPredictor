use uuid::Uuid;
use crate::api::Period;
use serde::{Deserialize, Serialize};

pub mod fetcher;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub id: Uuid,
    pub symbol: String,
    pub period: Period,
    pub data: String,
}
