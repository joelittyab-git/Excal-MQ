use net::socket::server::{data::SocketData, error::ServerSocketError};

use super::error::PrologueError;

pub trait Prologue{
     fn open_socket(&self)->Result<(), PrologueError>;
     async fn receive_data(&self)->Result<SocketData, PrologueError>;
}