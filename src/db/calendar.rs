use mongodb::{Database, bson::doc};
use futures::stream::TryStreamExt;

use crate::models::calendar::{Calendar, CalendarDocument, RedeemCode, RedeemCodeDocument};

pub async fn get_all_activities(db: &Database) -> mongodb::error::Result<Vec<Calendar>> {
    let collection = db.collection::<CalendarDocument>("calendar");
    let mut cursor = collection.find(doc! {"status": 1}, None).await?;

    let mut activities: Vec<Calendar> = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let title = result.title;
        let begin_time = result.begin_time;
        let end_time = result.end_time;
        let description = result.description;
        // transform ObjectId to String
        let customer_json = Calendar {
            _id: _id.to_string(),
            title,
            begin_time,
            end_time,
            description,
        };
        activities.push(customer_json);
    }


    Ok(activities)
}

pub async fn get_all_codes(db: &Database) -> mongodb::error::Result<Vec<RedeemCode>> {
    let collection = db.collection::<RedeemCodeDocument>("code");
    let mut cursor = collection.find(None, None).await?;

    let mut codes: Vec<RedeemCode> = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let code = result.code;
        let desc = result.desc;
        // transform ObjectId to String
        let customer_json = RedeemCode {
            _id: _id.to_string(),
            code,
            desc,
        };
        codes.push(customer_json);
    }


    Ok(codes)
}