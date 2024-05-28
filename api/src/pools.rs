use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("posts")]
pub struct Db(PgPool);
