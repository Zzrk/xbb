use mongodb::Database;
use futures::stream::TryStreamExt;

use crate::models::hero::{Hero, HeroDocument};

pub async fn get_all_heroes(db: &Database) -> mongodb::error::Result<Vec<Hero>> {
    let collection = db.collection::<HeroDocument>("hero");
    let mut cursor = collection.find(None, None).await?;

    let mut heroes: Vec<Hero> = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let name = result.name;
        let category = result.category;
        let star = result.star;
        let stages = result.stages;


        // transform ObjectId to String
        let customer_json = Hero {
            _id: _id.to_string(),
            category,
            name,
            star,
            stages,
        };
        heroes.push(customer_json);
    }


    Ok(heroes)
}