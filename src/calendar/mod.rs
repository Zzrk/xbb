use rocket::serde::json::{from_str, json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;

type Id = usize;

type CalendarInfoList = Mutex<Vec<CalendarInfo>>;
type CalendarInfoState<'r> = &'r State<CalendarInfoList>;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct CalendarInfo {
    id: Id,
    title: String,
    begin_time: String,
    end_time: String,
    description: String,
}

// 根据 id 查询数据
#[get("/<id>")]
async fn get(id: Id, list: CalendarInfoState<'_>) -> Option<Json<CalendarInfo>> {
    let list = list.lock().await;
    let info = list.get(id)?;

    Some(Json(info.clone()))
}

// 获取全部数据
#[get("/all")]
async fn get_all(list: CalendarInfoState<'_>) -> Json<Vec<CalendarInfo>> {
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
    let mock = r#"
        [
          {
            "id": 0,
            "title": "栽培计划",
            "begin_time": "2023-09-25 21:00:00Z",
            "end_time": "2023-10-15 16:00:00Z",
            "description": ""
          },
          {
            "id": 1,
            "title": "焚天之翼",
            "begin_time": "2023-09-26 02:00:00Z",
            "end_time": "2023-10-10 21:00:00Z",
            "description": ""
          },
          {
            "id": 2,
            "title": "远征金币翻倍",
            "begin_time": "2023-10-13 21:00:00Z",
            "end_time": "2023-10-15 21:00:00Z",
            "description": "活动期间, 开启远征宝箱, 获取金币奖励翻倍."
          }
        ]
    "#;
    // JSON 反序列化
    let list: Vec<CalendarInfo> = from_str(mock).unwrap();

    rocket::fairing::AdHoc::on_ignite("Calendar", |rocket| async {
        rocket
            .mount("/calendar", routes![get, get_all])
            .register("/calendar", catchers![not_found])
            .manage(CalendarInfoList::new(list))
    })
}
