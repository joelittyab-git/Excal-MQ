pub mod error;
pub mod data;

use std::{net::Ipv4Addr, time::Duration};

use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, net::TcpStream, time};

use error::ClientSocketError;
use super::data::ProtocolParser as ProtocolParse;

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
     /// Asynchronously receives data from the TCP stream.
     ///
     /// # Arguments
     ///
     /// * `buf` - A mutable byte buffer to store the received data.
     ///
     /// # Returns
     ///
     /// This function returns a `Result<usize, ClientSocketError>`, where:
     /// - `Ok(n)` returns the number of bytes read.
     /// - `Err(ClientSocketError)` if an error occurs during the read operation.
     ///
     /// # Errors
     ///
     /// - `ClientSocketError::IoError` if there's a network or stream issue.
     ///
     /// # Example
     ///
     /// ```rust
     /// let mut buf = vec![0; 1024];
     /// let bytes_read = socket.recv(&mut buf).await.unwrap();
     /// ```
     pub async fn recv(&mut self, buf: &mut [u8]) -> Result<usize, ClientSocketError> {
          let n = self.stream.read(buf).await?;
          Ok(n)
     }

     /// Closes the TCP connection gracefully.
     ///
     /// # Returns
     ///
     /// This function returns a `Result<(), ClientSocketError>`, where:
     /// - `Ok(())` if the connection is successfully closed.
     /// - `Err(ClientSocketError)` if an error occurs during the closing process.
     ///
     /// # Example
     ///
     /// ```rust
     /// socket.close().await.unwrap();
     /// ```
     pub async fn close(&mut self) -> Result<(), ClientSocketError> {
          self.stream.shutdown().await.map_err(|e| ClientSocketError::IoError { source: e })
     }

     /// Asynchronously attempts to send data with a timeout.
     ///
     /// # Arguments
     /// 
     /// * `data` - The string data to send.
     /// * `timeout_duration` - The duration after which the send operation will timeout.
     ///
     /// # Returns
     ///
     /// `Ok(())` if the data is successfully sent within the timeout.
     /// `Err(ClientSocketError)` if it times out or encounters an error.
     pub async fn send_with_timeout(&mut self, data: String, timeout_duration: Duration) -> Result<(), ClientSocketError> {
          match time::timeout(timeout_duration, self.stream.write(data.as_bytes())).await {
               Ok(Ok(_)) => Ok(()),
               Ok(Err(e)) => Err(ClientSocketError::IoError { source: e }),
               Err(_) => Err(ClientSocketError::TimeoutError{
                    message:"Request Timeout".to_string()
               }),
          }
     }

     /// Sends a framed message with a length prefix.
     /// Sends the frame implemented on [ProtocolParse]
     ///
     /// # Arguments
     ///
     /// * `data` - The message to send.
     ///
     /// # Returns
     ///
     /// Sends the length of the message followed by the actual data.
     pub async fn send_frame(&mut self, data: impl ProtocolParse) -> Result<(), ClientSocketError> {
          // buffer
          let bytes:Vec<u8> = match data.to_bytes() {
               Ok(b) => b,
               Err(e) => return Err(ClientSocketError::ProtocolParseError { source: e }),
          };

          match self.stream.write(&bytes).await{
               Ok(e) => return Ok(()),
               Err(s) => Err(ClientSocketError::IoError { source: s }),
          }
     }

     /// Receives a framed message with a length prefix.
     /// Receives the frame implemented on [ProtocolParse]
     /// Protocol standard tx. of data through socket
     ///
     /// # Returns
     ///
     /// The message data received after the length prefix.
     pub async fn recv_frame<T: Clone + ProtocolParse>(
          &mut self,
          protocol: &mut T
      ) -> Result<T, ClientSocketError> {
          let mut bytes: Vec<u8> = Vec::new();
          
          // Reading bytes from stream
          self.recv(&mut bytes).await.map_err(|e| ClientSocketError::from(e))?;
          
          // Parsing protocol data
          match protocol.from_raw(bytes) {

              Ok(parsed_data) =>{
                    protocol.clone_from(&parsed_data);
                    Ok(parsed_data) 
               },
              Err(e) => Err(ClientSocketError::ProtocolParseError { source: e }),
          }
     }

     /// Flushes the stream.
     ///
     /// # Returns
     ///
     /// A `Result` that returns `Ok(())` if the flush is successful, or `ClientSocketError` if an error occurs.
     ///
     /// # Example
     /// 
     /// ```rust
     /// socket.flush().await.unwrap();
     /// ```
     pub async fn flush(&mut self) -> Result<(), ClientSocketError> {
          self.stream.flush().await.map_err(|e| ClientSocketError::IoError { source: e })
     }

     /// Splits the TCP stream into a readable half and a writable half.
     ///
     /// # Returns
     ///
     /// A tuple containing the read half and write half of the TCP stream.
     ///
     /// # Example
     /// 
     /// ```rust
     /// let (read_half, write_half) = socket.split();
     /// ```
     pub fn split(self) -> (tokio::net::tcp::OwnedReadHalf, tokio::net::tcp::OwnedWriteHalf) {
          self.stream.into_split()
     }

     /// Gets the local address of the TCP stream.
     ///
     /// # Returns
     ///
     /// A `Result` that returns the local `SocketAddr` of the stream, or `ClientSocketError` if an error occurs.
     ///
     /// # Example
     /// 
     /// ```rust
     /// let local_addr = socket.get_local_addr().unwrap();
     /// ```
     pub fn get_local_addr(&self) -> Result<std::net::SocketAddr, ClientSocketError> {
          self.stream.local_addr().map_err(|e| ClientSocketError::IoError { source: e })
     }

     /// Gets the peer address of the TCP stream.
     ///
     /// # Returns
     ///
     /// A `Result` that returns the peer's `SocketAddr`, or `ClientSocketError` if an error occurs.
     ///
     /// # Example
     /// 
     /// ```rust
     /// let peer_addr = socket.get_peer_addr().unwrap();
     /// ```
     pub fn get_peer_addr(&self) -> Result<std::net::SocketAddr, ClientSocketError> {
          self.stream.peer_addr().map_err(|e| ClientSocketError::IoError { source: e })
     }

     /// Reads data from the stream until a specified delimiter is found.
     ///
     /// # Arguments
     ///
     /// * `delimiter` - A byte representing the delimiter.
     ///
     /// # Returns
     ///
     /// A `Result` containing the bytes read, or a `ClientSocketError` if an error occurs.
     ///
     /// # Example
     /// 
     /// ```rust
     /// let data = socket.read_until(b'\n').await.unwrap();
     /// ```
     pub async fn read_until(&mut self, delimiter: u8) -> Result<Vec<u8>, ClientSocketError> {
          // Wrap the TcpStream in a BufReader to use read_until
          let mut reader = BufReader::new(&mut self.stream);
          let mut buffer = Vec::new();
          reader.read_until(delimiter, &mut buffer).await.map_err(|e| ClientSocketError::IoError { source: e })?;
          Ok(buffer)
     }

     /// Reads all data from the stream until the connection is closed.
     ///
     /// # Returns
     ///
     /// A `Result` containing the bytes read, or a `ClientSocketError` if an error occurs.
     ///
     /// # Example
     /// 
     /// ```rust
     /// let data = socket.read_to_end().await.unwrap();
     /// ```
     pub async fn read_to_end(&mut self) -> Result<Vec<u8>, ClientSocketError> {
          let mut buffer = Vec::new();
          self.stream.read_to_end(&mut buffer).await.map_err(|e| ClientSocketError::IoError { source: e })?;
          Ok(buffer)
     }

     /// Checks if the socket is still connected by attempting a non-blocking peek operation.
     ///
     /// # Returns
     ///
     /// A `Result` that returns `Ok(true)` if the connection is still active, or `Ok(false)` if it is not.
     ///
     /// # Example
     /// 
     /// ```rust
     /// let is_connected = socket.is_connected().await.unwrap();
     /// ```
     pub async fn is_connected(&mut self) -> Result<bool, ClientSocketError> {
          let mut buf = [0u8; 1];
          match time::timeout(Duration::from_millis(500), self.stream.peek(&mut buf)).await {
          Ok(Ok(_)) => Ok(true),
          Ok(Err(_)) | Err(_) => Ok(false),
          }
     }

     /// Gracefully shuts down the TCP connection.
     ///
     /// # Returns
     ///
     /// A `Result` that returns `Ok(())` if the shutdown is successful, or `ClientSocketError` if an error occurs.
     ///
     /// # Example
     /// 
     /// ```rust
     /// socket.shutdown().await.unwrap();
     /// ```
     pub async fn shutdown(&mut self) -> Result<(), ClientSocketError> {
          self.stream.shutdown().await.map_err(|e| ClientSocketError::IoError { source: e })
     }
}