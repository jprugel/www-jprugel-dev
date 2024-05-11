use rocket_db_pools::diesel::prelude::*;

table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        date -> Text,
        body -> Text,
        published -> Bool,
    }
}
