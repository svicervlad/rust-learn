#[macro_use] extern crate rocket;

#[get("/<name>")]
fn index(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
