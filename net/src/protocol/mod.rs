/// The `interface` module contains definitions for the core components of the message transfer protocol.
/// It includes traits, enums, and structs used for handling and defining the protocol's functionality,
/// message structure, and various components involved in message handling.
///
/// This module provides the building blocks for implementing a message broker service by defining:
///
/// - **Traits**: Interfaces for message transfer and response handling.
///   - [`MessageTransferProtocol`]: Defines methods for subscribing, unsubscribing, publishing, pulling messages,
///     and managing protocol actions.
///   - [`MessageTransferProtocolResponse`]: Provides methods to retrieve status codes, headers, and storage
///     information from responses.
///   - [`MessageTransferProtocolPayload`]: Defines methods to get headers, messages, request types, and timestamps
///     from protocol payloads.
///
/// - **Enums**: Various enumerations used to specify message attributes and protocol details.
///   - [`MTPRequestType`]: Specifies different types of requests that can be made using the protocol.
///   - [`MTPHeaderUnit`]: Represents units within the protocol's header, including authentication, administration,
///     source, message, and publish information.
///   - [`MTPAuth`]: Represents different authentication methods used within the protocol.
///   - [`AuthSchemes`]: Defines the supported authentication schemes.
///   - `MTPManagerAction`: Defines actions that can be performed on the protocol, such as renaming or disposing
///     of clients.
///   - [`MessagePriority`]: Specifies priority levels of messages.
///   - [`MessageCategory`]: Defines the different categories a message can belong to.
///   - [`MessagePublish`]: Defines different ways a message can be published within the protocol.
///
/// - **Structs**: Data structures used to represent various aspects of the protocol.
///   - [`MTPHeaders`]: Represents the headers of a message, including header units, local storage, and a timestamp.
///   - [`MTPMessage`]: Represents the actual message content, including content type, priority, category, publish
///     method, and the message itself.
///
/// This module provides the foundational elements required to define and manage messages within a message broker
/// system. The components defined here are crucial for implementing and handling different protocol operations and
/// message processing scenarios.
pub mod interface;


pub mod error;