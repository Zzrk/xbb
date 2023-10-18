use std::fs;

use rocket::serde::json::{from_str, json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;

type MyEquipmentMutex = Mutex<MyEquipment>;
type MyEquipmentState<'r> = &'r State<MyEquipmentMutex>;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct MyEquipmentInfo {
    id: usize,
    name: String,
    count: usize,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct MyEquipment {
    item: Vec<MyEquipmentInfo>,
    fragment: Vec<MyEquipmentInfo>,
}

// 获取整装数据
#[get("/item")]
async fn get_item(list: MyEquipmentState<'_>) -> Json<Vec<MyEquipmentInfo>> {
    let all = list.lock().await;
    let list: Vec<MyEquipmentInfo> = all.item.clone();
    Json(list)
}

// 获取碎片数据
#[get("/fragment")]
async fn get_fragment(list: MyEquipmentState<'_>) -> Json<Vec<MyEquipmentInfo>> {
    let all = list.lock().await;
    let list: Vec<MyEquipmentInfo> = all.fragment.clone();
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
    let mock = fs::read_to_string("src/my_equipment/mock.json").expect("no such file");
    // JSON 反序列化
    let data: MyEquipment = from_str(mock.as_str()).unwrap();

    rocket::fairing::AdHoc::on_ignite("MyEquipment", |rocket| async {
        rocket
            .mount("/my_equipment", routes![get_item, get_fragment])
            .register("/my_equipment", catchers![not_found])
            .manage(MyEquipmentMutex::new(data))
    })
}
