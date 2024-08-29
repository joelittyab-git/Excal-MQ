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

use std::time::SystemTime;

use interface::{
     MessagePublish,
     MessageCategory,
     MessagePriority,
     ContentType,
     MTPManagerAction,
     MTPHeaderUnit,
     MTPStatusCode,
     MessageTransferProtocolResponse
};

/// [`MTPResponse`] represents the response returned from operations performed in the message transfer protocol.
/// It includes a status code, headers, and storage information that describe the result of the protocol operation.
pub struct MTPResponse {
     /// The status code indicating the outcome of the protocol operation.
     status_code: MTPStatusCode,
 
     /// The headers associated with the response, containing metadata and other relevant information.
     headers: MTPHeaders,
 
     /// The storage associated with the response, which may include additional data or resources.
     storage: MTPStorage,
 }
 
impl MTPResponse {
     /// Constructs a new `MTPResponse` instance with the provided status code, headers, and storage.
     ///
     /// # Arguments
     ///
     /// * `status` - The status code indicating the result of the operation.
     /// * `headers` - The headers associated with the response.
     /// * `storage` - The storage data associated with the response.
     ///
     /// # Returns
     ///
     /// A new instance of `MTPResponse`.
     ///
     /// # Example
     ///
     /// ```
     /// let response = MTPResponse::construct(status_code, headers, storage);
     /// ```
     fn construct(status: MTPStatusCode, headers: MTPHeaders, storage: MTPStorage) -> Self {
         Self {
             status_code: status,
             headers,
             storage,
         }
     }
 }
 
impl MessageTransferProtocolResponse for MTPResponse {
     /// Retrieves the status code from the `MTPResponse`.
     ///
     /// # Returns
     ///
     /// The status code associated with the response.
     fn get_status_code(&self) -> MTPStatusCode {
         self.status_code.clone()
     }
 
     /// Retrieves the headers from the `MTPResponse`.
     ///
     /// # Returns
     ///
     /// An `Option` containing the headers associated with the response.
     /// Returns `None` if there are no headers.
     fn get_headers(&self) -> Option<MTPHeaders> {
         Some(self.headers.clone())
     }
 
     /// Retrieves the storage data from the `MTPResponse`.
     ///
     /// # Returns
     ///
     /// An `Option` containing the storage data associated with the response.
     /// Returns `None` if there is no storage data.
     fn get_storage(&self) -> Option<MTPStorage> {
         Some(self.storage.clone())
     }
 }
 


/// `MTPManagerActions` represents a collection of management actions that can be performed.
pub struct MTPManagerActions {
     actions: Vec<MTPManagerAction>,
}
 
 /// `MTPMessage` represents the actual content of a message within the protocol.
 /// It encapsulates the message data along with metadata that describes its type,
 /// priority, category, and how it should be published.
 ///
 /// This struct is used to define the core content of a message and includes various fields
 /// to support different formats and handling requirements as specified by the protocol.
 ///
 /// ## Fields
 ///
 /// ### `content_type`
 ///
 /// Specifies the format or type of the message content. This field determines how the message
 /// content is structured or encoded, such as JSON or XML.
 ///
 /// - **Type**: `ContentType`
 /// - **Description**: Indicates the content format of the message, guiding how the message should
 ///   be interpreted or processed.
 ///
 /// ### `priority`
 ///
 /// Defines the priority level of the message, indicating its importance or urgency. This field
 /// helps in managing message handling and processing based on the message's priority.
 ///
 /// - **Type**: `MessagePriority`
 /// - **Description**: Specifies the priority of the message, which can influence how and when
 ///   the message is handled by the system.
 ///
 /// ### `category`
 ///
 /// Specifies the category or type of the message, which helps in classifying the message based
 /// on its purpose or content. This field provides context for the message's role within the system.
 ///
 /// - **Type**: `MessageCategory`
 /// - **Description**: Indicates the category of the message, providing information about its
 ///   functional role or content type.
 ///
 /// ### `publish`
 ///
 /// Defines how the message should be published or disseminated within the system. This field
 /// specifies the target or scope of the message distribution, such as to all recipients, a specific
 /// recipient, or a group of recipients.
 ///
 /// - **Type**: `MessagePublish`
 /// - **Description**: Determines the publishing method for the message, guiding its delivery
 ///   to the intended recipients.
 ///
 /// ### `message`
 ///
 /// Contains the actual content of the message as a string. This field holds the data or text
 /// that constitutes the main part of the message.
 ///
 /// - **Type**: `String`
 /// - **Description**: Stores the content of the message in string format, representing the
 ///   core data to be communicated.
 ///
 /// ## Example
 ///
 /// Here is an example of how `MTPMessage` might be used:
 ///
 /// ```rust
 /// pub struct MTPMessage {
 ///     content_type: ContentType,
 ///     priority: MessagePriority,
 ///     category: MessageCategory,
 ///     publish: MessagePublish,
 ///     message: String,
 /// }
 ///
 /// fn process_message(mtp_message: MTPMessage) {
 ///     // Access and process the message content and metadata
 ///     println!("Message Content: {}", mtp_message.message);
 ///     println!("Content Type: {:?}", mtp_message.content_type);
 ///     println!("Priority: {:?}", mtp_message.priority);
 ///     println!("Category: {:?}", mtp_message.category);
 ///     println!("Publish Method: {:?}", mtp_message.publish);
 /// }
 ///
 /// // Create an example message
 /// let example_message = MTPMessage {
 ///     content_type: ContentType::JSON,
 ///     priority: MessagePriority::High,
 ///     category: MessageCategory::COMMAND,
 ///     publish: MessagePublish::ALL,
 ///     message: String::from("This is a command message"),
 /// };
 ///
 /// // Process the example message
 /// process_message(example_message);
 /// ```
 ///
 /// In this example, an `MTPMessage` instance is created with specific content and metadata,
 /// and the `process_message` function demonstrates how to access and handle the message's
 /// various attributes.
 pub struct MTPMessage {
     content_type:ContentType,
     priority:MessagePriority,
     category:MessageCategory,
     publish:MessagePublish,
     message:String
 }


 /// `MTPHeaders` represents the headers of a message within the protocol.
/// It includes various components that provide metadata and contextual information
/// about the message, such as header units, local storage, and a timestamp.
///
/// This struct is used to encapsulate all the header-related information necessary
/// for processing and managing messages within the protocol.
///
/// ## Fields
///
/// ### `headers`
///
/// A vector of `MTPHeaderUnit` instances that constitute the header of the message.
/// Each `MTPHeaderUnit` contains specific types of information, such as authentication data,
/// source details, message details, and publish information. This vector allows for a flexible
/// and extensible way to include multiple header units in the message header.
///
/// - **Type**: `Vec<MTPHeaderUnit>`
/// - **Description**: Stores all header units related to the message, providing a comprehensive
///   collection of metadata and control information.
///
/// ### `local`
///
/// An instance of `MTPStorage` that contains additional data or storage specific to the message.
/// This field allows for the inclusion of extra storage elements that are relevant to the message
/// but are not part of the primary header units.
///
/// - **Type**: `MTPStorage`
/// - **Description**: Provides storage for additional data or metadata associated with the message,
///   separate from the primary header units.
///
/// ### `timestamp`
///
/// The timestamp representing the time at which the message was created or received.
/// This field is used to record when the message was processed or when its header was generated,
/// allowing for time-based tracking and ordering of messages.
///
/// - **Type**: `SystemTime`
/// - **Description**: Records the exact time of message creation or reception, useful for
///   time-based processing and tracking of message flow.
///
/// ## Example
///
/// Here is an example of how `MTPHeaders` might be used:
///
/// ```rust
/// use std::time::SystemTime;
/// 
/// // Define a header unit
/// let header_unit = MTPHeaderUnit::Authentication {
///     key: MTPAuth::Authorization {
///         scheme: AuthSchemes::Bearer,
///     },
///     value: String::from("token_value"),
/// };
///
/// // Define local storage
/// let storage = MTPStorage {
///     items: vec![StorageCell {
///         key: String::from("key1"),
///         value: String::from("value1"),
///     }],
/// };
///
/// // Create an instance of MTPHeaders
/// let headers = MTPHeaders {
///     headers: vec![header_unit],
///     local: storage,
///     timestamp: SystemTime::now(),
/// };
///
/// // Accessing fields
/// println!("Timestamp: {:?}", headers.timestamp);
/// ```
///
/// In this example, an `MTPHeaders` instance is created with a single header unit, local storage, and
/// the current system time as the timestamp. The fields of the struct can be accessed for further
/// processing or management of messages.
pub struct MTPHeaders {
     headers: Vec<MTPHeaderUnit>,
     local: MTPStorage,
     timestamp: SystemTime,
 }

 
/// `MTPStorage` represents a collection of storage cells, which can be used to store additional
/// data or pointers within the protocol.
pub struct MTPStorage {
     items: Vec<StorageCell>,
 }
 
 /// `StorageCell` represents an individual storage unit within [`MTPStorage`].
 /// Containts a key value pair for storing local data and caching information
 pub struct StorageCell {
     key: String,
     value: String,
 }
 

/// Clone implementation for [MTPStorage]
impl Clone for MTPStorage{
     fn clone(&self) -> Self {
          Self { items: self.items.clone() }
     }
}

/// Clone implementation for [StorageCell]
impl Clone for StorageCell{
     fn clone(&self) -> Self {
          Self { key: self.key.clone(), value: self.value.clone() }
     }
}

 
/// Clone implementation for [MTPHeaders]
impl Clone for MTPHeaders{
     fn clone(&self) -> Self {
         Self { headers: self.headers.clone(), local: self.local.clone(), timestamp: self.timestamp.clone() }
     }
}

/// Clone implementation for [MTPHeaderUnit]
impl Clone for MTPHeaderUnit {
     fn clone(&self) -> Self {
          match self {
               Self::Authentication { key, value } => Self::Authentication { key: key.clone(), value: value.clone() },
               Self::Administration { action } => Self::Administration { action: action.clone() },
               Self::Source { source } => Self::Source { source: source.clone() },
               Self::Message { id, timestamp, priority, category, content_type } => Self::Message { id: id.clone(), timestamp: timestamp.clone(), priority: priority.clone(), category: category.clone(), content_type: content_type.clone() },
               Self::MessagePublish { queue, to } => Self::MessagePublish { queue: queue.clone(), to: to.clone() },
          }
    }
}

/// Clone implementation for [MTPManagerAction]
impl Clone for MTPManagerAction{
     fn clone(&self) -> Self {
          match self {
               Self::Rename => Self::Rename,
               Self::Authorize => Self::Authorize,
               Self::Reject => Self::Reject,
               Self::Dispose => Self::Dispose,
               Self::AccessorModify => Self::AccessorModify,
          }
    }
}

/// CLone implementation for [MTPStatusCode]
impl Clone for MTPStatusCode{
     fn clone(&self) -> Self {
         match self {
             Self::Success0 => Self::Success0,
             Self::Error1(arg0) => Self::Error1(arg0.clone()),
         }
     }
 }
 