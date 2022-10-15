#[macro_use] extern crate rocket;
use rocket::fs::FileServer;

#[get("/<name>", rank = 2)]
fn index(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", FileServer::from("dist").rank(1))
    .mount("/", routes![index])
}
