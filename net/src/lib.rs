/// A module providing higher-level abstractions over [`std::net::TcpListener`] and [`std::net::TcpStream`].
///
/// This module wraps socket instances to facilitate various client and server socket operations.
/// It offers abstractions for TCP networking, making it easier to work with sockets in a more
/// convenient and higher-level manner. The module includes implementations for:
///
/// - Creating and managing TCP listeners and streams
/// - Handling client-server communication
/// - Managing different networking protocols and configurations
///
/// The abstractions provided here aim to simplify socket operations by encapsulating common tasks
/// and providing a more user-friendly interface for networking operations.
pub mod socket;


pub mod protocol;