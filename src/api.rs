use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id:     String, 
    pub author_id:      String, 
    pub chat_id:        String, 
    pub content:        String, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id:             String, 
    pub name:           String, 
    pub description:    String, 
    pub creation_date:  i64, 
    pub picture:        String, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub id:             String, 
    pub name:           String, 
    pub description:    String, 
    pub creation_date:  i64, 
    pub picture:        String, 
    pub nsfw:           bool, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub chat_id:        String, 
    pub name:           String, 
    pub description:    String, 
    pub creation_date:  i64, 
    pub picture:        String, 
    // ignored if server is nsfw
    pub nsfw:           bool, 
    pub server_id:      String, 
}
