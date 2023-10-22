use mongodb::Database;
use mongodb::bson::doc;
use futures::stream::TryStreamExt;

use crate::models::equipment::{Equipment, EquipmentDocument};

pub async fn get_all_equipments(db: &Database) -> mongodb::error::Result<Vec<Equipment>> {
    let collection = db.collection::<EquipmentDocument>("equipment");
    let mut cursor = collection.find(None, None).await?;

    let mut equipments: Vec<Equipment> = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let category = result.category;
        let name = result.name;
        let quality = result.quality;
        let access = result.access;
        let description = result.description;
        let synthesis = result.synthesis;
        let level = result.level;

        // transform ObjectId to String
        let customer_json = Equipment {
            _id: _id.to_string(),
            category,
            name,
            quality,
            access,
            description,
            synthesis,
            level,
        };
        equipments.push(customer_json);
    }


    Ok(equipments)
}

pub async fn get_all_equipment_items(db: &Database) -> mongodb::error::Result<Vec<Equipment>> {
    let collection = db.collection::<EquipmentDocument>("equipment");
    let mut cursor = collection.find(doc! { "category": "item" }, None).await?;

    let mut equipments: Vec<Equipment> = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let category = result.category;
        let name = result.name;
        let quality = result.quality;
        let access = result.access;
        let description = result.description;
        let synthesis = result.synthesis;
        let level = result.level;

        // transform ObjectId to String
        let customer_json = Equipment {
            _id: _id.to_string(),
            category,
            name,
            quality,
            access,
            description,
            synthesis,
            level,
        };
        equipments.push(customer_json);
    }


    Ok(equipments)
}

pub async fn get_all_equipment_fragments(db: &Database) -> mongodb::error::Result<Vec<Equipment>> {
    let collection = db.collection::<EquipmentDocument>("equipment");
    let mut cursor = collection.find(doc! { "category": "fragment" }, None).await?;

    let mut equipments: Vec<Equipment> = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let category = result.category;
        let name = result.name;
        let quality = result.quality;
        let access = result.access;
        let description = result.description;
        let synthesis = result.synthesis;
        let level = result.level;

        // transform ObjectId to String
        let customer_json = Equipment {
            _id: _id.to_string(),
            category,
            name,
            quality,
            access,
            description,
            synthesis,
            level,
        };
        equipments.push(customer_json);
    }


    Ok(equipments)
}