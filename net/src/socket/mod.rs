/// Module for server-side functionality in the `excal-mq` system.
///
/// This module contains all instances and abstractions related to the server side
/// of the `excal-mq` message queue system. It includes tools and utilities for
/// handling server socket operations, offering higher-level abstractions over
/// [std::net::TcpListener] to support various protocols used by `excal-mq`.
///
/// # Features
///
/// - **Server Socket Operations**: Provides functionality for managing and operating
///   server sockets, including accepting incoming connections and handling data.
/// - **Protocol Abstractions**: Includes abstractions for different network protocols
///   used by `excal-mq` to facilitate communication between components.
///
/// # See Also
///
/// - [`client`] for client-side abstractions and operations.
/// - [`data`] for definitions related to data transmitted over TCP streams.
pub mod server;

/// Module for client-side functionality in the `excal-mq` system.
///
/// This module encompasses all instances and abstractions related to the client side
/// of the `excal-mq` message queue system. It provides higher-level abstractions for
/// client socket operations, facilitating communication with server components.
///
/// # Features
///
/// - **Client Socket Operations**: Includes tools for establishing and managing
///   client connections, sending and receiving data.
/// - **Communication Abstractions**: Provides abstractions to interact with server
///   components effectively.
///
/// # See Also
///
/// - [`server`] for server-side abstractions and operations.
/// - [`data`] for definitions related to data transmitted over TCP streams.
pub mod client;

/// Module for handling data transmitted over TCP streams.
///
/// This module contains all instances and enums related to the data payload transmitted
/// in TCP stream communications. It defines the structures and types used for representing
/// and processing data sent and received over the network.
///
/// # Features
///
/// - **Data Representations**: Includes enums and structures for different types of
///   data, such as raw bytes, UTF-8, and UTF-16 encoded text.
/// - **Payload Handling**: Provides tools for encoding and decoding data payloads
///   to facilitate communication between server and client.
///
/// # See Also
///
/// - [`server`] for server-side functionality and operations.
/// - [`client`] for client-side functionality and operations.
pub mod data;
