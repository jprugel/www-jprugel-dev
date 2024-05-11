use rocket_db_pools::diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub body: String,
}
