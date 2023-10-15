#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, xbb!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
