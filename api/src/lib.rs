pub mod pools;
mod models;
mod schema;
mod utility;

use rocket::{
    get,
    serde::{
        json::Json,
        Serialize,
    },
};

use rocket_db_pools::{
    Connection,
    diesel::prelude::*,
};

use pools::Db;
use models::Article;
use schema::posts;
use utility::ClientArticle;

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
