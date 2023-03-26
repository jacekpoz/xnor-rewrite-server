use rocket_db_pools::{Database, sqlx::SqlitePool};

#[derive(Database)]
#[database("db")]
pub struct Db(SqlitePool);
