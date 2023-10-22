#[macro_use]
extern crate rocket;
use dotenv::dotenv;

mod my_equipment;
mod db;
mod models;
mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, xbb!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(db::init())
        .mount("/", routes![index])
        .attach(routes::calendar::stage())
        .attach(routes::equipment::stage())
        .attach(routes::hero::stage())
        .attach(my_equipment::stage())
}
