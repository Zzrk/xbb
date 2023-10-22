use mongodb::Database;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

use crate::models::calendar::Calendar;
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
            .mount("/calendar", routes![get_all])
            .register("/calendar", catchers![not_found])
    })
}
