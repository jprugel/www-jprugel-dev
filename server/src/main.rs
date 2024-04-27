use std::env;
use std::io;

use rocket::{fs::FileServer, get, launch, response, routes, tokio::fs};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{prelude::*, PgPool};
use sycamore::prelude::*;
use client::components::article::*;

#[derive(Database)]
#[database("posts")]
struct Db(PgPool);

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
struct Post {
    id: i32,
    title: String,
    date: String,
    body: String,
    published: bool,
}

table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        date -> Text,
        body -> Text,
        published -> Bool,
    }
}

#[get("/")]
async fn index(mut db: Connection<Db>) -> io::Result<response::content::RawHtml<String>> {
    let index_html = String::from_utf8(fs::read("./client/dist/index.html").await?)
        .expect("index.html should be valid utf-8");

    let rendered = sycamore::render_to_string(|| {
        view! {
            client::App {}
        }
    });

    let db_info: Vec<Post> = posts::table
        .select(Post::as_select())
        .load(&mut db)
        .await
        .unwrap();

    let example_article = sycamore::render_to_string(|| {
        view! {
            client::components::article::Article(
                id=db_info.get(0).unwrap().id.clone(),
                title=db_info.get(0).unwrap().title.clone(),
                date=db_info.get(0).unwrap().date.clone(),
                body=db_info.get(0).unwrap().body.clone(),
            ) {}
        }
    });

    let index_html = index_html.replace("%sycamore.body", &rendered);
    let index_html = index_html.replace("%db_info.body", &example_article);

    Ok(response::content::RawHtml(index_html))
}

#[launch]
fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "full");
    println!(
        "what: {}",
        std::env::current_dir().unwrap().to_str().unwrap()
    );

    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index])
        .mount("/", FileServer::from("./client/dist"))
}
