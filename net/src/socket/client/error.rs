use std::io::Error;

/// An enum to represent the different errors that can occur for the [crate::socket::client::ClientSocket] instance
pub enum ClientSocketError{
     /// Indicates that the error caused is due to I/O operations by tcp_listner and other I/O object
     IoError{
          /// The underlying I/O Error
          source:Error
     }
}

/// From implementation to typecast [std::io::Error] to [ClientSocketError]
impl From<Error> for ClientSocketError{
     fn from(value: Error) -> Self {
          ClientSocketError::IoError { source: value }
     }
}