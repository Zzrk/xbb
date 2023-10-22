use mongodb::Database;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

use crate::models::hero::Hero;
use crate::db::hero;

// 获取全部数据
#[get("/all")]
async fn get_all(db: &State<Database>) -> Result<Json<Vec<Hero>>, String> {
    match hero::get_all_heroes(&db).await {
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
    rocket::fairing::AdHoc::on_ignite("Hero", |rocket| async {
        rocket
            .mount("/hero", routes![get_all])
            .register("/hero", catchers![not_found])
    })
}
