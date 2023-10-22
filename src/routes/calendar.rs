use mongodb::Database;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

use crate::models::calendar::{Calendar, RedeemCode};
use crate::db::calendar;

// 获取全部数据
#[get("/all")]
async fn get_all(db: &State<Database>) -> Result<Json<Vec<Calendar>>, String> {
    match calendar::get_all_activities(&db).await {
        Ok(_customer_docs) => Ok(Json(_customer_docs)),
        Err(_error) => {
            println!("{:?}", _error);
            return Err(_error.to_string());
        }
    }
}

// 获取全部兑换码
#[get("/all_codes")]
async fn get_all_codes(db: &State<Database>) -> Result<Json<Vec<RedeemCode>>, String> {
    match calendar::get_all_codes(&db).await {
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
    rocket::fairing::AdHoc::on_ignite("Calendar", |rocket| async {
        rocket
            .mount("/calendar", routes![get_all, get_all_codes])
            .register("/calendar", catchers![not_found])
    })
}
