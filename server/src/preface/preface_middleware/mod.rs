pub mod error;

use error::MiddlewareError;
use net::socket::server::data::SocketData;

pub trait Middleware {
     fn parse(&self, socket_data:SocketData)->Result<SocketData, MiddlewareError>;
}

