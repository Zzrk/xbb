use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarDocument {
    pub _id: ObjectId,
    pub title: String,
    pub begin_time: String,
    pub end_time: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    pub _id: String,
    pub title: String,
    pub begin_time: String,
    pub end_time: String,
    pub description: String,
}