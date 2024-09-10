pub mod error;
pub mod data;

use std::net::Ipv4Addr;

use tokio::{io::AsyncWriteExt, net::TcpStream};

use error::ClientSocketError;

/// A simple socket for wrapping over async standard tcp stream
/// Simplifies the tco_stream by returning data in an enclosed entity
/// 
/// # Fields
/// 
/// ~ `srtream`: The tcp stream object of this socket
pub struct ClientSocket{
     stream:TcpStream
}

impl ClientSocket {
     /// Asynchronously attempts to establish a connection to a server running on `localhost` at the specified port.
     /// Establishes a connection with a lisrtener on the localhost at the specified port.
     /// # Arguments
     /// 
     /// * `port` - A 16-bit unsigned integer representing the port number on `localhost` where the client will attempt to establish a TCP connection.
     ///
     /// # Returns
     /// 
     /// A `Result` that on success contains an instance of `Self` representing the established connection, or on failure returns a [`ClientSocketError`].
     ///
     /// - On success, it returns `Ok(Self)` where `Self` is the current type containing the TCP connection stream.
     /// - On failure, it returns `Err(ClientSocketError)` if the connection attempt failed, specifically with the variant `ClientSocketError::IoError`, including the underlying I/O error as the `source`.
     ///
     /// # Errors
     ///
     /// This function can fail for various reasons, such as:
     /// - The server not running on the specified `localhost` port.
     /// - Network issues preventing the connection.
     /// - The port being blocked or already in use.
     ///
     /// In these cases, the function will return an error of type `ClientSocketError::IoError`, which wraps the underlying I/O error that occurred during the connection attempt.
     ///
     /// # Example
     ///
     /// ```rust
     /// let socket = ClientSocket::connect(8080);     //Connect at "localhost:8080"
     /// ```
     ///
     /// # Async
     /// 
     /// This function is asynchronous and requires `await` to be called. It uses asynchronous I/O to establish the connection, which means the function does not block the thread while waiting for the connection to be established.
     ///
     /// # See Also
     /// 
     /// - [`TcpStream::connect`](https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html#method.connect) for more details on how the underlying TCP connection works.
     pub async fn connect(port:u16)->Result<Self, ClientSocketError>{
          // localhost
          let localhost = Ipv4Addr::new(127, 0, 0, 1);

          // attempts to connect to localhost and port
          let stream = match TcpStream::connect((localhost, port)).await{
               Ok(s) => s,
               Err(e) => {
                    return Result::Err(ClientSocketError::IoError { source:  e});
               },
          };

          Ok(Self{
               stream
          })

     }

     /// Asynchronously sends data over a TCP connection represented by the current instance.
     ///
     /// # Arguments
     ///
     /// * `data` - A `String` containing the data that you want to send over the TCP connection. The string is converted into bytes before being transmitted.
     ///
     /// # Returns
     ///
     /// This function returns a `Result<(), ClientSocketError>`, where:
     /// - `Ok(())` is returned if the data is successfully sent.
     /// - `Err(ClientSocketError)` is returned if an error occurs during the sending process. This could happen for various reasons, such as:
     ///   - A network failure.
     ///   - The connection being closed prematurely.
     ///
     /// # Errors
     ///
     /// If the underlying `write` operation fails, this function returns a `ClientSocketError`. Specifically, this might happen if:
     /// - The TCP stream is closed or interrupted.
     /// - The connection encounters a network error during transmission.
     /// The error is propagated via the `?` operator, which handles the error and converts it into a `ClientSocketError`.
     ///
     /// # Example
     ///
     /// ```rust
     /// let socket = ClientSocket::connect(8080).await.unwrap();
     /// socket.send("This is some data").await;
     /// ```
     ///
     /// # Async
     ///
     /// This function is asynchronous and must be awaited. It uses `await` to perform a non-blocking write operation to the TCP stream, ensuring that the program can continue running other tasks while the data is being sent.
     ///
     /// # See Also
     ///
     /// - [`write`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncWriteExt.html#method.write) from the `AsyncWriteExt` trait for more details on the underlying asynchronous write operation.
     pub async fn send(&mut self, data:String)->Result<(), ClientSocketError>{
          let _ = self.stream.write(data.as_bytes()).await?;

          return Ok(());
     }
}