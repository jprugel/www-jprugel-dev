use rocket_db_pools::diesel::prelude::*; 
use rocket::serde::{Serialize, Deserialize};
use crate::schema::*;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub body: String,
}
