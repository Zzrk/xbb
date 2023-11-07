#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use rocket::fs::FileServer;

mod db;
mod models;
mod routes;
mod fairings;

#[get("/")]
fn index() -> &'static str {
    "Hello, xbb!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(db::init())
        .attach(fairings::cors::CORS)
        .mount("/", routes![index])
        .mount("/", FileServer::from("public/"))
        .attach(routes::calendar::stage())
        .attach(routes::equipment::stage())
        .attach(routes::hero::stage())
}
