use mongodb::Database;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

use crate::models::equipment::Equipment;
use crate::db::equipment;

// 获取全部数据
#[get("/all")]
async fn get_all(db: &State<Database>) -> Result<Json<Vec<Equipment>>, String> {
    match equipment::get_all_equipments(&db).await {
        Ok(_customer_docs) => Ok(Json(_customer_docs)),
        Err(_error) => {
            println!("{:?}", _error);
            return Err(_error.to_string());
        }
    }
}

// 获取整装数据
#[get("/item")]
async fn get_item(db: &State<Database>) -> Result<Json<Vec<Equipment>>, String> {
    match equipment::get_all_equipment_items(&db).await {
        Ok(_customer_docs) => Ok(Json(_customer_docs)),
        Err(_error) => {
            println!("{:?}", _error);
            return Err(_error.to_string());
        }
    }
}

// 获取整装数据
#[get("/fragment")]
async fn get_fragment(db: &State<Database>) -> Result<Json<Vec<Equipment>>, String> {
    match equipment::get_all_equipment_fragments(&db).await {
        Ok(_customer_docs) => Ok(Json(_customer_docs)),
        Err(_error) => {
            println!("{:?}", _error);
            return Err(_error.to_string());
        }
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Equipment", |rocket| async {
        rocket
            .mount("/equipment", routes![get_all, get_item, get_fragment])
            .register("/equipment", catchers![not_found])
    })
}
