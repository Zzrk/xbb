use std::fs;

use rocket::serde::json::{from_str, json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;

type HeroInfoList = Mutex<Vec<HeroInfo>>;
type HeroInfoState<'r> = &'r State<HeroInfoList>;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct HeroStage {
    stage: String,
    equipments: Vec<String>,
    level: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct HeroInfo {
    name: String,
    category: String,
    star: u8,
    stages: Vec<HeroStage>,
}

// 获取全部数据
#[get("/all")]
async fn get_all(list: HeroInfoState<'_>) -> Json<Vec<HeroInfo>> {
    let list = list.lock().await;
    Json(list.clone())
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    // 模拟数据
    let mock = fs::read_to_string("src/hero/mock.json").expect("no such file");
    // JSON 反序列化
    let list: Vec<HeroInfo> = from_str(mock.as_str()).unwrap();

    rocket::fairing::AdHoc::on_ignite("Hero", |rocket| async {
        rocket
            .mount("/hero", routes![get_all])
            .register("/hero", catchers![not_found])
            .manage(HeroInfoList::new(list))
    })
}
