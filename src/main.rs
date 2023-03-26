#[macro_use] 
extern crate rocket;
mod api;
mod db;
mod result;
mod routes;

use rocket_db_pools::Database;

use dotenvy::dotenv;

use db::Db;

use routes::*;

#[launch]
fn rocket() -> _ {

    dotenv().ok();

    //let sql_flavour = std::env::var("SQL_FLAVOUR").expect("choose your sql flavour using $SQL_FLAVOUR");
    //let conn_str = 
    //    if sql_flavour != "sqlite" { 
    //        std::env::var("DATABASE_URL")
    //        .expect("set the env var $DATABASE_URL")
    //        .as_str()
    //    } else { "" };

    //let pool = match sql_flavour.to_lowercase().as_str() {
    //    "sqlite" => SqlitePool::connect(""),
    //    "pg" | "pgsq" | "postgres" |
    //    "postgresql" => PgPool::connect(""), 
    //};

    rocket::build()
        .attach(Db::init())
        .mount("/", routes![send_message, get_message])
}
