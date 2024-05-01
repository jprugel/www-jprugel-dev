use std::env;
use rocket::{fs::FileServer, launch, routes};
use rocket_db_pools::Database;
use server::{index, Db};

#[launch]
fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "full");

    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index])
        .mount("/", FileServer::from("./client/dist"))
}
