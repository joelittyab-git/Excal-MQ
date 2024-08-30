/// Module containing error types related to [`crate::socket::server::ServerSocket`] 
/// and its various implementations.
///
/// This module defines the error types used within the server-side functionality of
/// the `excal-mq` system. These errors encompass issues that may arise during
/// socket operations, such as connection handling, data reading, and more.
///
/// # Features
///
/// - **Error Types**: Provides various error types and enums to handle different error conditions
///   that can occur within the server socket operations.
/// - **Error Handling**: Facilitates robust error handling by defining specific errors and their
///   causes related to the server socket functionality.
///
/// # See Also
///
/// - [`server`] for the server-side functionality and operations.
/// - [`SocketData`] for handling data received from TCP streams.
/// - [`data`] for data serialization and deserialization.
///
pub mod error;

/// Module related to data being transferred to and from [`crate::socket::server::ServerSocket`].
/// This includes serialization and deserialization processes.
///
/// This module deals with the data payload being sent and received over TCP streams.
/// It provides the mechanisms for encoding and decoding data, ensuring that data can be
/// properly transmitted and interpreted by different components of the `excal-mq` system.
///
/// # Features
///
/// - **Data Serialization**: Functions and structures for converting data into a format suitable
///   for transmission over a network.
/// - **Data Deserialization**: Functions and structures for interpreting and converting received
///   data back into its original form.
/// - **Data Formats**: Handles various data formats and encodings, such as raw bytes, UTF-8, and
///   UTF-16.
///
/// # See Also
///
/// - [`crate::socket::server`] for server-side socket operations that utilize this data.
/// - [`SocketData`] for the structure of data handled by server sockets.
/// - [`crate::socket::server::error`] for error types related to data handling and socket operations.
///
pub mod data;

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use std::net::{Ipv4Addr, SocketAddr};

use error::ServerSocketError;
use data::SocketData;

use crate::socket::data::Type;
use crate::socket::data::Data;
use crate::socket::data::Endian;



/// A simple socket for wrapping over async standard tcp listener
/// Simplifies the tcp_listener by returning data in an enclosed entity
/// 
/// # Fields
/// 
/// ~ `port`: The port in which the socket listens
/// ~ `host`: The host at which the socket is running
/// ~ `tcp_listener`: The tcp listener object of this socket
pub struct ServerSocket{
     port:u16,
     host:Ipv4Addr,
     tcp_listener:TcpListener
}

impl ServerSocket{
     /// Creates a new `ServerSocket` instance bound to the specified port.
     ///
     /// This asynchronous function initializes a TCP listener bound to the given port
     /// on the localhost (127.0.0.1). It returns a `ServerSocket` instance on success,
     /// or a `ServerSocketError` if an error occurs during the binding process.
     ///
     /// # Parameters
     ///
     /// - `port`:
     ///   - A `u16` value representing the port number to which the TCP listener should be bound.
     ///     The port must be within the valid range for TCP ports (1 to 65535).
     ///
     /// # Returns
     ///
     /// - `Ok(Self)`:
     ///   - On success, returns an instance of `ServerSocket` that has been successfully bound
     ///     to the specified port. The `ServerSocket` instance includes the port number, host address,
     ///     and the `TcpListener` bound to the specified port.
     /// - `Err(ServerSocketError)`:
     ///   - On failure, returns a `ServerSocketError` indicating why the binding operation failed.
     ///     This may include IO errors such as the port being in use or other underlying issues.
     ///
     /// # Example
     ///
     /// ```rust
     /// use your_crate::ServerSocket;
     ///
     /// #[tokio::main]
     /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
     ///     let port: u16 = 8080;
     ///     match ServerSocket::new(port).await {
     ///         Ok(server) => {
     ///             println!("Server started on port {}", server.port);
     ///         }
     ///         Err(e) => {
     ///             eprintln!("Failed to start server: {:?}", e);
     ///         }
     ///     }
     ///     Ok(())
     /// }
     /// ```
     ///
     /// # Errors
     ///
     /// This function may return an error if:
     /// - The port is already in use.
     /// - There is a problem with network configuration or permissions.
     /// - Other IO-related errors occur while attempting to bind the listener.
     ///
     /// # Notes
     ///
     /// - Ensure that the specified port is not already in use by another application.
     /// - Make sure that your application has the necessary permissions to bind to the specified port.
     ///
     /// # See Also
     ///
     /// - [`ServerSocketError`] for details on the possible errors.
     /// - [`TcpListener`] for information on TCP listener behavior and usage.
     /// ```rust
     /// pub async fn new(port: u16) -> Result<Self, ServerSocketError> {
     ///     //localhost
     ///     let localhost = Ipv4Addr::new(127, 0, 0, 1);
     ///     let tcp_listener = TcpListener::bind((localhost, port)).await?; // ServerSocketError::IoError{source:<Error>}
     ///
     ///     Ok(ServerSocket {
     ///         port,
     ///         host: localhost,
     ///         tcp_listener
     ///     })
     /// }
     /// ```
     pub async fn new(port: u16) -> Result<Self, ServerSocketError> {
          //localhost
          let localhost = Ipv4Addr::new(127, 0, 0, 1);
          let tcp_listener = TcpListener::bind((localhost, port)).await?; // ServerSocketError::IoError{source:<Error>}
     
          Ok(ServerSocket {
          port,
          host: localhost,
          tcp_listener
          })
     }
 

     /// Asynchronously reads data from an incoming TCP connection and parses it based on the specified data type.
     ///
     /// This function listens for an incoming TCP connection, reads the data from the socket, and parses it according to the
     /// specified [Type]. The data is processed differently depending on whether it is raw bytes, UTF-16, or UTF-8 encoded.
     ///
     /// # Parameters
     ///
     /// - `data_type`:
     ///   - A [Type] enum that specifies how to interpret the data read from the socket. The variants can be:
     ///     - `[Type::Bytes]`: Treats the data as raw bytes.
     ///     - `Type::Utf16`: Treats the data as UTF-16 encoded text.
     ///     - `Type::Utf8`: Treats the data as UTF-8 encoded text.
     ///
     /// # Returns
     ///
     /// - `Ok(SocketData)`:
     ///   - On success, returns a `SocketData` instance containing the parsed data and the address of the socket.
     ///   - The `SocketData` will include:
     ///     - The address from which the data was received.
     ///     - The data parsed according to the `data_type` specified, which can be raw bytes, UTF-16 text, or UTF-8 text.
     /// - `Err(ServerSocketError)`:
     ///   - On failure, returns a [ServerSocketError] This indicates issues such as problems accepting the connection,
     ///     reading from the stream, or data parsing errors.
     ///
     /// # Example
     ///
     /// ```rust
     /// // Assume `self` is an instance with a `tcp_listener`
     /// match self.read_incoming(Type::Utf8).await {
     ///     Ok(socket_data) => {
     ///         println!("Received data from {}: {:?}", socket_data.address(), socket_data.data());
     ///     }
     ///     Err(e) => {
     ///         eprintln!("Error reading incoming data: {:?}", e);
     ///     }
     /// }
     /// ```
     ///
     /// # Errors
     ///
     /// This function may return an error in the following situations:
     /// - Failure to accept a new connection on the TCP listener.
     /// - Failure to read data from the socket stream.
     /// - Errors in parsing the data to the specified encoding type.
     ///
     /// # Notes
     ///
     /// - Ensure that the TCP listener is properly initialized and running.
     /// - The `Data::to_utf16_string` function must be correctly implemented to handle UTF-16 conversion.
     ///
     /// # See Also
     ///
     /// - [`Type`] for the different data types you can specify for parsing.
     /// - [`SocketData`] for the structure of the data returned.
     /// - [`ServerSocketError`] for details on the possible errors.
     /// ```rust
     /// pub async fn read_incoming(&self, data_type: Type) -> Result<SocketData, ServerSocketError> {
     ///     // Function implementation
     /// }
     /// ```
     pub async fn read_incoming(&self, data_type: Type) -> Result<SocketData, ServerSocketError> {
          //Awaits for async tcp connection
          let (mut stream, addr): (TcpStream, SocketAddr) = self.tcp_listener.accept().await?;

          //buffer initialization
          let mut buf: Vec<u8> = Vec::new();
     
          // Reading socket data
          stream.read(&mut buf).await?;
     
          // Matching the type passed for parsing data for a specific encoding
          return match data_type {
               Type::Bytes => Ok(SocketData::new(addr, Data::Bytes(buf))),
               Type::Utf16 => {
                    let utf16_string = Data::to_utf16_string(&buf, Endian::Big).await;
                    Ok(SocketData::new(addr, Data::Utf16(utf16_string)))
               },
               Type::Utf8 => {
                    let utf8_string = String::from_utf8_lossy(&buf).to_string();
                    Ok(SocketData::new(addr, Data::Utf8(utf8_string)))
               }
          }
     }
 

     /// Asynchronously reads UTF-8 encoded data from an incoming TCP connection.
     ///
     /// This function wraps the `read_incoming` method, specifying `Type::Utf8` to indicate
     /// that the data should be interpreted as UTF-8 encoded text. It reads data from the
     /// TCP stream, parses it as UTF-8, and returns the result.
     ///
     /// # Returns
     ///
     /// - `Ok(SocketData)`:
     ///   - On success, returns a [SocketData] instance containing the parsed UTF-8 data
     ///     and the address of the socket from which the data was received.
     /// - `Err(ServerSocketError)`:
     ///   - On failure, returns a [ServerSocketError] indicating why the operation failed.
     ///     This may include issues such as problems with accepting the connection, reading
     ///     from the stream, or parsing errors.
     ///
     /// # Example
     ///
     /// ```rust
     /// use your_crate::ServerSocket;
     ///
     /// #[tokio::main]
     /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
     ///     let server = ServerSocket::new(8080).await?;
     ///
     ///     match server.read().await {
     ///         Ok(socket_data) => {
     ///             println!("Received data: {:?}", socket_data);
     ///         }
     ///         Err(e) => {
     ///             eprintln!("Error reading data: {:?}", e);
     ///         }
     ///     }
     ///     Ok(())
     /// }
     /// ```
     ///
     /// # Errors
     ///
     /// This function may return an error if:
     /// - The `read_incoming` function fails.
     /// - There are issues with the TCP connection or stream.
     /// - The data cannot be properly parsed as UTF-8.
     ///
     /// # Notes
     ///
     /// - The function assumes that the data being read should be UTF-8 encoded. If the data
     ///   is encoded in a different format, use `read_incoming` with the appropriate [Type] variant.
     ///
     /// # See Also
     ///
     /// - [`read_incoming`] for more details on how data is read and parsed.
     /// - [`SocketData`] for the structure of the data returned.
     /// - [`ServerSocketError`] for details on the possible errors.
     /// ```rust
     /// pub async fn read(&self) -> Result<SocketData, ServerSocketError> {
     ///     return self.read_incoming(Type::Utf8).await;
     /// }
     /// ```
     pub async fn read(&self) -> Result<SocketData, ServerSocketError> {
          return self.read_incoming(Type::Utf8).await;
     }
 
}
