use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ClientArticle {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub hook: String,
    pub body: String,
}
