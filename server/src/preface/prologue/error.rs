use std::fmt::Display;

use net::socket::server::error::ServerSocketError;

pub enum PrologueError {
     SocketError{
          source:ServerSocketError,
          info:String
     },

     MiddlewareError{

     }
}

impl Display for PrologueError{
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
     }
}