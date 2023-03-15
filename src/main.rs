use std::net::Ipv6Addr;

use rocket::{get, launch, routes, Config};

#[launch]
fn rocket() -> _ {
    let config = Config {
        port: 8080,
        address: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(),
        ..Config::default()
    };
    rocket::custom(&config).mount("/", routes![hello])
}

#[get("/<_..>")]
fn hello() -> &'static str {
    "Hello world!"
}
