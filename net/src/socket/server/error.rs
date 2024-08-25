use std::io::Error;

/// An enum to represent the different errors that can occur for the [crate::socket::server::Socket] instance
pub enum ServerSocketError{
     /// Indicates that the error caused is due to I/O operations by tcp_listner and other I/O object
     IoError{
          /// The underlying I/O Error
          source:Error
     }
}

/// From implementation to type cast [std::io::Error] to [ServerSocketError]
impl From<Error> for ServerSocketError{
     fn from(value: Error) -> Self {
          return Self::IoError{source:value};
     }
}