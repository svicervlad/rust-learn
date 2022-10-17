#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket_async_compression::Compression;

#[get("/<name>", rank = 2)]
fn index(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
async fn rocket() -> _ {
    let server = rocket::build()
    .mount("/", FileServer::from("dist").rank(1))
    .mount("/", routes![index]);

    if cfg!(debug_assertions) {
        server
    } else {
        server.attach(Compression::fairing())
    }
}
