use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentCount {
    name: String,
    count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentDocument {
    pub _id: ObjectId,
    pub category: String,
    pub name: String,
    pub quality: String,
    pub access: Option<Vec<String>>,
    pub description: String,
    pub synthesis: Option<Vec<EquipmentCount>>,
    pub level: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equipment {
    pub _id: String,
    pub category: String,
    pub name: String,
    pub quality: String,
    pub access: Option<Vec<String>>,
    pub description: String,
    pub synthesis: Option<Vec<EquipmentCount>>,
    pub level: Option<usize>,
}