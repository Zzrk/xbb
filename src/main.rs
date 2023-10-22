#[macro_use]
extern crate rocket;
use dotenv::dotenv;

mod equipment;
mod hero;
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
        .attach(equipment::stage())
        .attach(hero::stage())
        .attach(my_equipment::stage())
}
