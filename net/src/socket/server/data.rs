use std::net::SocketAddr;

use crate::socket::data::Data;

/// Represents data received from an incoming TCP connection on the server.
///
/// `SocketData` wraps all relevant information for a specific socket connection,
/// including the address of the incoming stream and the data contained in the stream.
///
/// # Fields
///
/// - `address`:
///   - The address of the incoming stream as a [`std::net::SocketAddr`]. This represents
///     the remote address from which the data was received.
///
/// - `data`:
///   - The data contained in the incoming TCP stream, represented by the [`Data`] enum.
///     This includes the actual payload that was read from the stream, which can be
///     in various formats such as raw bytes, UTF-8, or UTF-16 encoded text.
///
/// # Example
///
/// ```rust
/// use std::net::SocketAddr;
/// use your_crate::data::Data;
/// use your_crate::SocketData;
///
/// // Example creation of a `SocketData` instance
/// let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
/// let data = Data::Utf8("Hello, world!".to_string());
/// let socket_data = SocketData::new(address, data);
///
/// println!("Received data from {}: {:?}", socket_data.address, socket_data.data);
/// ```
///
/// # See Also
///
/// - [`Data`] for the various types of data that can be contained in a `SocketData`.
/// - [`ServerSocket`] for the server-side component that uses `SocketData` to handle incoming connections.
pub struct SocketData {
     address: SocketAddr,
     data: Data,
 }
 
 impl SocketData {
     /// Creates a new `SocketData` instance with the specified address and data.
     ///
     /// # Arguments
     ///
     /// * `address` - The [`std::net::SocketAddr`] of the incoming stream. This is the address
     ///   from which the data was received.
     ///
     /// * `data` - The [`Data`] containing the deserialized data after reading from the
     ///   TCP stream. This encapsulates the payload read from the stream.
     ///
     /// # Returns
     ///
     /// Returns a `SocketData` instance containing the provided `address` and `data`.
     ///
     /// # Example
     ///
     /// ```rust
     /// use std::net::SocketAddr;
     /// use your_crate::data::Data;
     /// use your_crate::SocketData;
     ///
     /// let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
     /// let data = Data::Utf8("Example data".to_string());
     /// let socket_data = SocketData::new(address, data);
     /// ```
     pub fn new(address: SocketAddr, data: Data) -> Self {
         Self {
             address,
             data,
         }
     }
 }
 

