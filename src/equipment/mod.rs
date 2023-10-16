use std::fs;

use rocket::serde::json::{from_str, json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;

type EquipmentInfoList = Mutex<Vec<EquipmentInfo>>;
type EquipmentInfoState<'r> = &'r State<EquipmentInfoList>;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct EquipmentCount {
    name: String,
    count: usize,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct EquipmentInfo {
    category: String,
    name: String,
    quality: String,
    access: Option<Vec<String>>,
    description: String,
    synthesis: Option<Vec<EquipmentCount>>,
    level: Option<usize>,
}

// 获取全部数据
#[get("/all")]
async fn get_all(list: EquipmentInfoState<'_>) -> Json<Vec<EquipmentInfo>> {
    let list = list.lock().await;
    Json(list.clone())
}

// 获取整装数据
#[get("/item")]
async fn get_item(list: EquipmentInfoState<'_>) -> Json<Vec<EquipmentInfo>> {
    let all = list.lock().await;
    let list: Vec<EquipmentInfo> = all
        .clone()
        .into_iter()
        .filter(|x| x.category == "item")
        .collect();
    Json(list)
}

// 获取整装数据
#[get("/fragment")]
async fn get_fragment(list: EquipmentInfoState<'_>) -> Json<Vec<EquipmentInfo>> {
    let all = list.lock().await;
    let list: Vec<EquipmentInfo> = all
        .clone()
        .into_iter()
        .filter(|x| x.category == "fragment")
        .collect();
    Json(list)
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
    let mock = fs::read_to_string("src/equipment/mock.json").expect("no such file");
    // JSON 反序列化
    let list: Vec<EquipmentInfo> = from_str(mock.as_str()).unwrap();

    rocket::fairing::AdHoc::on_ignite("Equipment", |rocket| async {
        rocket
            .mount("/equipment", routes![get_all, get_item, get_fragment])
            .register("/equipment", catchers![not_found])
            .manage(EquipmentInfoList::new(list))
    })
}
