use std::net::Ipv6Addr;

use rocket::{get, launch, routes, Config};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(&Config {
            port: 8080,
            address: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(),
            ..Config::default()
        })
        .mount("/", routes![hello])
}

#[get("/<_..>")]
fn hello() -> &'static str {
    "Hello world!"
}
