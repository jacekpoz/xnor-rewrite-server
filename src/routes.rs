use crate::api::*;
use crate::db::Db;
use crate::result::Result;

use rocket::serde::json::Json;

use rocket_db_pools::{Connection, sqlx};

//use ulid::Ulid;

#[post("/message", data = "<message>")]
pub async fn send_message(
    mut db: Connection<Db>, 
    message: Json<Message>, 
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO messages 
        VALUES (?, ?, ?, ?) 
        "#, 
        message.message_id, 
        message.author_id, 
        message.chat_id, 
        message.content, 
    )
    .execute(&mut *db)
    .await?;
    println!("{:?}", message);

    Ok(())
}

#[get("/message/<id>")]
pub async fn get_message(
    mut db: Connection<Db>, 
    id: &str, 
) -> Result<Json<Message>> {
    let id = id.to_string();
    let message = sqlx::query_as!(Message,
        r#"
        SELECT message_id, author_id, chat_id, content as "content!" FROM messages
        WHERE message_id = ?;
        "#, 
        id
    )
    .fetch_one(&mut *db)
    .await?;

    Ok(Json(message))
}
    

//#[get("/server/<id>")]
//pub async fn get_server(
//    mut db: Connection<Db>, 
//    id: &str, 
//) -> Result<Json<Server>> {
//    let id = Ulid::from_string(id);
//    match id {
//        Ok(id) => {
//            let server = sqlx::query(
//                r#"
//                SELECT * FROM servers
//                WHERE id = ?
//                "#
//            )
//            .bind(id.to_string())
//            .fetch_one(&mut *db)
//            .await?;
//
//            Ok(Json(Server {
//                id: Ulid::from_string(&server.get::<String, &str>("id")).unwrap(), 
//                name: server.get::<String, &str>("name"), 
//                description: server.get::<String, &str>("description"), 
//                creation_date: server.get::<i64, &str>("creation_date"), 
//                picture: server.get::<String, &str>("picture"), 
//                nsfw: server.get::<bool, &str>("nsfw"), 
//            }))
//        }, 
//        Err(_) => Err(Error::InvalidUlid)
//    } 
//}
