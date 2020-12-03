use std::error::Error as StdError;
use std::fmt;

pub struct Server {
    id: String,
    pub connection_string: String,
    pub name: String,
}

pub struct Channel {
    pub id: String,
    server_id: String,
    pub name: String,
}

pub struct User {
    pub id: String,
    server_id: String,
    pub name: String,
}

pub struct Post {
    pub id: String,
    pub content: String,
    pub channel_id: String,
    pub author_id: String,
}


#[derive(Debug)]
pub enum Error {
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Other(err) => write!(f, "error: {}", err),
        }
    }
}

impl StdError for Error {}

impl std::convert::From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}

trait Store {
    fn add_server(&self, conn: String, name: String) -> Result<&Server, Error>;
    fn get_server(&self, name: String) -> Result<&Server, Error>;
    
    fn add_channel(&self, server: Server, name: String, id: Option<String>);
    fn get_channel();

    fn add_user();
    fn get_user();

    fn add_post();
    fn get_post();
}
