pub mod models;
pub mod pools;
pub mod schema;
pub mod utility;

use rocket::tokio::io;
use std::sync::{Arc, Mutex};

use crate::models::*;
use crate::pools::*;
use crate::schema::*;
use crate::utility::*;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{get, response, tokio::fs};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use sycamore::prelude::*;

#[get("/")]
pub async fn index() -> io::Result<response::content::RawHtml<String>> {
    // Create the document to be hydrated.
    let index_html = String::from_utf8(fs::read("./client/dist/index.html").await?)
        .expect("index.html should be valid utf-8");
    let rendered = Arc::new(Mutex::new(String::new()));
    let clone_a = rendered.clone();
    let clone_b = rendered.clone();
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let local = tokio::task::LocalSet::new();
        rt.block_on(async{
            local.run_until(async move{
                tokio::task::spawn_local(async move{
                    get_mutex_val(clone_a).push_str(&sycamore::render_to_string_await_suspense(|| {
                    view! {
                        client::App
                    }
                }).await);
                }).await.unwrap();
            }).await;
        });
    }).await?;

    //Insert the app into the index.html in the wasm package.
    let index_html = index_html.replace("%sycamore.body", &get_mutex_val(clone_b));

    //Return the final html call.
    Ok(response::content::RawHtml(index_html))
}

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Feed {
    articles: Vec<ClientArticle>,
}

impl Feed {
    fn new(articles: Vec<ClientArticle>) -> Self {
        Self {
            articles,
        }
    }
}

// Gets all articles from the server.
#[get("/feed")]
pub async fn feed(mut db: Connection<Db>) -> Json<Feed> {
    let db_info: Vec<Article> = posts::table
        .filter(posts::published.eq(true))
        .select(Article::as_select())
        .load(&mut db)
        .await
        .expect("Failed to connect to database");

    let client = db_info.into_iter()
        .map(|x| {
            let hook = x.body.split_inclusive("  ").collect::<Vec<&str>>()[1];

            ClientArticle {
                id: x.id as usize,
                title: x.title,
                date: x.date,
                hook: hook.to_string(),
                body: x.body,
            }
        })
        .collect();

    Json(Feed::new(client))
}

fn get_mutex_val(target: Arc<Mutex<String>>) -> String {
    target.lock().unwrap().to_string()
}
