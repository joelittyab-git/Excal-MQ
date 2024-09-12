use std::io::Error;

use crate::protocol::error::ProtocolError;

/// An enum to represent the different errors that can occur for the [crate::socket::client::ClientSocket] instance
pub enum ClientSocketError{
     /// Indicates that the error caused is due to I/O operations by tcp_listner and other I/O object
     IoError{
          /// The underlying I/O Error
          source:Error
     },


     /// Indicates that the socket has timed out of transcating (I/O) data
     TimeoutError{
          message:String
     },


     /// Indiacates that an error has occured during the parsing of data
     ProtocolParseError{
          source:ProtocolError
     }


}

/// From implementation to typecast [std::io::Error] to [ClientSocketError]
impl From<Error> for ClientSocketError{
     fn from(value: Error) -> Self {
          ClientSocketError::IoError { source: value }
     }
}