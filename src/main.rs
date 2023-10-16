#[macro_use]
extern crate rocket;

mod calendar;
mod equipment;

#[get("/")]
fn index() -> &'static str {
    "Hello, xbb!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(calendar::stage())
        .attach(equipment::stage())
}
