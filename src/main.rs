use rocket::{get, launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}

#[get("/<_..>")]
fn hello() -> &'static str {
    "Hello world!"
}
