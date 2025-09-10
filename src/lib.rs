mod cmd;
pub use cmd::Command;

mod db;
use db::{Db, DbDropGuard};

mod parse;
use parse::{Parse, ParseError};

mod frame;
pub use frame::Frame;

mod connection;
pub use connection::Connection;

pub mod server;

mod shutdown;
use shutdown::Shutdown;

pub const DEFAULT_PORT: u16 = 6379;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>; 