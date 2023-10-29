use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarDocument {
    pub _id: ObjectId,
    pub title: String,
    pub begin_time: String,
    pub end_time: String,
    pub description: Vec<String>,
    pub status: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    pub _id: String,
    pub title: String,
    pub begin_time: String,
    pub end_time: String,
    pub description: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedeemCodeDocument {
    pub _id: ObjectId,
    pub code: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedeemCode {
    pub _id: String,
    pub code: String,
    pub desc: String,
}