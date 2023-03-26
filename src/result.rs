use rocket::response;
use rocket::Request;
use rocket::http::ContentType;
use rocket::http;
use rocket_db_pools::sqlx;
use rocket::serde::json::Json;

use std::io::Cursor;

pub enum Error {
    /// connection died
    Disconnect, 
    /// failed to parse ulid
    InvalidUlid, 
    /// something wasn't found
    NotFound, 
    /// anything else that I didn't bother to implement
    IDK, 
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<sqlx::Error> for Error {
    fn from(er: sqlx::Error) -> Self {
        match er {
            sqlx::Error::Configuration(..) | 
            sqlx::Error::Io(..) | 
            sqlx::Error::Tls(..) | 
            sqlx::Error::Protocol(..) | 
            sqlx::Error::PoolTimedOut | 
            sqlx::Error::PoolClosed | 
            sqlx::Error::WorkerCrashed => Error::Disconnect, 
            sqlx::Error::RowNotFound | 
            sqlx::Error::TypeNotFound { type_name: _ } |
            sqlx::Error::ColumnNotFound(..) => Error::NotFound, 
            _ => Error::IDK, 
        }
    }
}


#[rocket::async_trait]
impl<'r> response::Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let string = Json(self).to_string();

        let status = http::Status::InternalServerError;

        response::Response::build()
            .header(ContentType::JSON)
            .sized_body(string.len(), Cursor::new(string))
            .status(status)
            .ok()
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
