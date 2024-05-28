use rocket::{
    launch, 
    routes
};

use rocket_db_pools::Database;

use api::{
    feed,
    pools::*,
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![feed])
}
