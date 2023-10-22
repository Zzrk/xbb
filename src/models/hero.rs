use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeroStage {
    stage: String,
    equipments: Vec<String>,
    level: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeroDocument {
    pub _id: ObjectId,
    pub name: String,
    pub category: String,
    pub star: u8,
    pub stages: Vec<HeroStage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hero {
    pub _id: String,
    pub name: String,
    pub category: String,
    pub star: u8,
    pub stages: Vec<HeroStage>,
}