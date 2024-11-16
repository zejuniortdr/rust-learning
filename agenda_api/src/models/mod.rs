// src/models/mod.rs
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgendaItem {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub date: String,
}
