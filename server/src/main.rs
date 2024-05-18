use std::env;
use rocket::{fs::FileServer, launch, routes};
use rocket_db_pools::Database;
use server::{index, feed};
use server::pools::Db;

#[launch]
fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "full");

    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index, feed])
        .mount("/", FileServer::from("./client/dist"))
}
