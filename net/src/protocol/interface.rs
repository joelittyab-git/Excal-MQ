use std::net::SocketAddr;
use std::time::SystemTime;

use super::{
     MTPManagerActions,
     MTPHeaders,
     MTPStorage,
     MTPMessage,

     error::ProtocolError
};


/// [`MessageTransferProtocol`] defines the main interface for a message transfer protocol system.
/// It includes methods for subscribing to queues, unsubscribing, publishing messages,
/// pulling messages, pinging the server, and managing actions.
/// 
/// All the protocol actions result in an entity that implements [`MessageTransferProtocolResponse`] 
pub trait MessageTransferProtocol {
    type Response: MessageTransferProtocolResponse;
    type Message: MessageTransferProtocolPayload;

    /// Subscribes to a specified queue.
    /// 
    /// # Arguments
    /// * `queue` - The identifier of the queue to subscribe to.
    ///
    /// # Returns
    /// A result containing a response or an error.
    fn subscribe(&self, queue: String) -> Result<Self::Response, ProtocolError>;

    /// Unsubscribes from the queue with the passed identifier.
    ///
    /// # Returns
    /// A result containing a response or an error.
    fn unsubscribe(&self, queue: String) -> Result<Self::Response, ProtocolError>;

    /// Publishes a message to the queue.
    ///
    /// # Arguments
    /// * `message` - The message to be published.
    ///
    /// # Returns
    /// A result containing a response or an error.
    fn publish(&self, message: Self::Message) -> Result<Self::Response, ProtocolError>;

    /// Pulls a message from the queue.
    ///
    /// # Returns
    /// A result containing a response or an error.
    fn pull(&self) -> Result<Self::Response, ProtocolError>;

    /// Pings the server to check status
    ///
    /// # Returns
    /// A result containing a response or an error.
    fn ping(&self) -> Result<Self::Response, ProtocolError>;

    /// Manages the protocol actions.
    ///
    /// # Arguments
    /// * `actions` - A set of actions to be managed.
    ///
    /// # Returns
    /// A result containing a response or an error.
    fn manage(&self, actions: MTPManagerActions) -> Result<Self::Response, ProtocolError>;
}

/// [`MessageTransferProtocolResponse`] represents the response returned by methods of the [`MessageTransferProtocol`] trait.
/// This trait provides methods to access the status code, headers, and storage information from the response.
///
/// The trait allows you to extract critical information from the response object to handle protocol interactions.
///
/// ## Methods
///
/// ### `get_status_code`
///
/// Retrieves the status code from the response. The status code indicates the result of the operation performed by the protocol.
/// 
/// # Returns
/// 
/// - `MTPStatusCode`: The status code of the response, which conveys the outcome of the operation. This value is always returned without the possibility of an error.
///
/// ### `get_headers`
///
/// Retrieves the headers from the response. Headers provide additional metadata or control information relevant to the response.
/// 
/// # Returns
/// 
/// - `Option<MTPHeaders>`: An `Option` where `Some(MTPHeaders)` contains the headers of the response, and `None` indicates that no headers are present.
///
/// ### `get_storage`
///
/// Retrieves the storage information from the response. This information may include additional data or pointers that are
/// relevant to the operation or response.
/// 
/// # Returns
/// 
/// - `Option<MTPStorage>`: An `Option` where `Some(MTPStorage)` contains the storage information from the response, and `None` indicates that no storage information is available.
///
/// ## Example
///
/// Here's an example implementation of `MessageTransferProtocolResponse`:
///
/// ```rust
/// struct MyResponse {
///     status_code: MTPStatusCode,
///     headers: Option<MTPHeaders>,
///     storage: Option<MTPStorage>,
/// }
///
/// impl MessageTransferProtocolResponse for MyResponse {
///     fn get_status_code(&self) -> MTPStatusCode {
///         self.status_code.clone()
///     }
///
///     fn get_headers(&self) -> Option<MTPHeaders> {
///         self.headers.clone()
///     }
///
///     fn get_storage(&self) -> Option<MTPStorage> {
///         self.storage.clone()
///     }
/// }
/// ```
/// 
/// In this example, `MyResponse` implements the `MessageTransferProtocolResponse` trait by providing concrete
/// implementations for each of the methods, where headers and storage are optional.
pub trait MessageTransferProtocolResponse {
     /// Retrieves the status of the response object
     fn get_status_code(&self) -> MTPStatusCode;

     /// Retrieves the headers passed along with the reponse.
     fn get_headers(&self) -> Option<MTPHeaders>;

     /// Retrieced the local storage (with headers)
     fn get_storage(&self) -> Option<MTPStorage>;
}

/// ```text
/// +------------------------------------------------+
/// |                 MTP-Payload                    |
/// +------------------------------------------------+
/// |                   Header                       |
/// |  +------------------------------------------+  |
/// |  |              Header Units                |  |
/// |  |  +---------------+  +---------------+    |  |
/// |  |  | Header Unit 1 |  | Header Unit 2 | ...|  |
/// |  |  +---------------+  +---------------+    |  |
/// |  +------------------------------------------+  |
/// |  +------------------------------------------+  |
/// |  |                Storage                   |  |
/// |  |  +-------------+  +-------------+        |  |
/// |  |  | Storage 1   |  | Storage 2   |  ...   |  |
/// |  |  +-------------+  +-------------+        |  |
/// |  +------------------------------------------+  |
/// +------------------------------------------------+
/// |                      Body                      |
/// |  +------------------------------------------+  |
/// |  |               Message Part               |  |
/// |  +------------------------------------------+  |
/// +------------------------------------------------+
/// ```
/// 
/// The `MessageTransferProtocolPayload` represents the actual data being sent within the protocol.
/// It contains methods to retrieve the headers, message, request type, and timestamp.
pub trait MessageTransferProtocolPayload {
     /// Retrieves the headers from the MTPPayload instance
     ///
     /// # Returns
     /// An option having [`MTPHeaders`] (list of headers passed with request)
     fn get_headers(&self) -> Option<MTPHeaders>;

     /// Retrieved the message (body) of the payload.
     ///
     /// # Returns
     /// An option containing a [`MTPMessage`] instance
     fn get_message(&self) -> Option<MTPMessage>;

     /// Retrieves the type of request of the payload
     ///
     /// # Returns
     /// An [`MTPRequestType`], the type of the request
     fn get_request(&self) -> MTPRequestType;

     /// Retrieves the time stamp of the request
     ///
     /// # Returns
     /// An option of the tinmestamp passed with the [MTPHeaders]
     fn get_timestamp(&self) -> Option<SystemTime>;
}

/// `MTPRequestType` defines the types of requests that can be made using the protocol in a message broker service.
/// These request types represent different operations that clients can perform within the message broker system.
///
/// Each variant corresponds to a specific action or operation that the message broker supports. Understanding these request
/// types is crucial for implementing the protocol and handling various client interactions within the message broker.
///
/// ## Variants
///
/// ### `Subscribe`
///
/// Represents a request to subscribe to a specific message queue. When a client sends this request, it is indicating its interest
/// in receiving and publishing messages from the specified queue.
///
/// ### `Unsubscribe`
///
/// Represents a request to unsubscribe from a message queue. This is used when a client no longer wishes to receive or publish
/// messages from the queue it was previously subscribed to.
///
/// ### `Publish`
///
/// Represents a request to publish a message to a specified queue. The client sends this request with the message content,
/// and the message broker routes the message to the appropriate queue for processing.
///
/// ### `Pull`
///
/// Represents a request to pull a message from the queue. This is typically used by clients to retrieve messages that have
/// been queued for them. The message broker will return a message from the queue as per the client's request.
///
/// ### `Ping`
///
/// Represents a request to check the health or connectivity of the message broker. This is a way for clients to verify
/// that the broker is responsive and operational. Responds with the status of the server
///
/// ### `Manage`
///
/// Represents a request for performing management actions on the message broker. This can include operations such as
/// renaming queues, authorizing users, or modifying access permissions. This request type is used for administrative tasks
/// that affect the message broker's configuration and operations.
///
/// ## Example
///
/// Here is an example of how `MTPRequestType` might be used in a message broker service:
///
/// ```rust
/// fn handle_request(request_type: MTPRequestType) {
///     match request_type {
///         MTPRequestType::Subscribe => {
///             // Handle subscription logic
///         },
///         MTPRequestType::Unsubscribe => {
///             // Handle unsubscription logic
///         },
///         MTPRequestType::Publish => {
///             // Handle message publication logic
///         },
///         MTPRequestType::Pull => {
///             // Handle message retrieval logic
///         },
///         MTPRequestType::Ping => {
///             // Handle health check or connectivity logic
///         },
///         MTPRequestType::Manage => {
///             // Handle management actions
///         },
///     }
/// }
/// ```
/// 
/// In this example, `handle_request` uses a `match` statement to determine how to process each type of request, enabling the
/// message broker to appropriately handle different client interactions based on the request type.

pub enum MTPRequestType {

     /// To subscribe to a message queue
     Subscribe,

     /// To unsubsribe from a message queue
     Unsubscribe,

     /// To publish a message to a specified queue
     Publish,

     /// To pull messages from a specified queue
     Pull,

     /// To ping the server to get the server status
     Ping,

     /// To perform manger functions on the queue 
     /// Only valid if the client has their respoective permission
     Manage,
}

/// `MTPStatusCode` represents the various status codes that can be returned in a protocol response.
pub enum MTPStatusCode {
    Success0,
    Error1(ProtocolError)
}

/// `MTPHeaderUnit` represents individual units within the protocol's header.
/// Each unit encapsulates different types of information that are part of the header section of a protocol message.
/// These units provide metadata and control information that is essential for managing and processing messages within the protocol.
///
/// ## Variants
///
/// ### `Authentication`
///
/// Represents authentication-related information within the header. This unit contains details used for verifying the
/// identity of the sender or ensuring secure access.
///
/// - `key`: An [`MTPAuth`] enum indicating the type of authentication used (e.g., token or authorization scheme).
/// - `value`: A [`String`] containing the authentication data or token.
///
/// ### `Administration`
///
/// Represents administrative actions or commands related to the message broker's management. This unit is used for
/// performing various administrative tasks or conveying control commands.
///
/// - `action`: An [`MTPManagerAction`] enum specifying the type of management action (e.g., renaming a queue or authorizing a user).
///
/// ### `Source`
///
/// Represents information about the source of the message. This unit contains details about where the message originated,
/// which can be useful for routing or logging purposes.
///
/// - `source`: A [`SocketAddr`] representing the address of the source sending the message.
///
/// ### `Message`
///
/// Represents details about the message being transmitted. This unit contains metadata about the message, such as its
/// identifier, timestamp, priority, category, and content type.
///
/// - `id`: A [`String`] representing the unique identifier of the message.
/// - `timestamp`: A [`SystemTime`] representing the time when the message was created or sent.
/// - `priority`: A [`MessagePriority`] enum indicating the importance level of the message (e.g., high or low).
/// - `category`: A [`MessageCategory`] enum categorizing the type of message (e.g., event, command, or error).
/// - `content_type`: A [`ContentType`] enum specifying the format of the message content (e.g., JSON or XML).
///
/// ### `MessagePublish`
///
/// Represents information about the message publishing operation. This unit is used to specify details about where the
/// message should be published within the message broker system.
///
/// - `queue`: A [`String`] specifying the identifier of the queue to which the message is being published.
/// - `to`: A [`MessagePublish`] enum indicating the target of the publication (e.g., all subscribers or specific groups).
///
/// ## Example
///
/// Here is an example of how `MTPHeaderUnit` might be used in practice:
///
/// ```rust
/// fn process_header_unit(header_unit: MTPHeaderUnit) {
///     match header_unit {
///         MTPHeaderUnit::Authentication { key, value } => {
///             // Handle authentication information
///         },
///         MTPHeaderUnit::Administration { action } => {
///             // Handle administrative actions
///         },
///         MTPHeaderUnit::Source { source } => {
///             // Handle source information
///         },
///         MTPHeaderUnit::Message { id, timestamp, priority, category, content_type } => {
///             // Handle message details
///         },
///         MTPHeaderUnit::MessagePublish { queue, to } => {
///             // Handle message publishing details
///         },
///     }
/// }
/// ```
/// 
/// In this example, `process_header_unit` uses a `match` statement to process different types of header units, allowing
/// for specific handling based on the type of information contained in each unit.
pub enum MTPHeaderUnit {

     /// All headers pertaining to authentication of the user
     /// Includes token from foreign security services
     Authentication {
          key: MTPAuth,
          value: String,
     },

     /// All headers pertaining to manager actions on a queue
     /// Only clients with specific roles have access to this, otherwise this header is ignored
     Administration {
          action: MTPManagerAction,
     },

     /// All headers pertaining to the source of the request [`SocketAddr`] object
     Source {
          source: SocketAddr,
     },

     /// All headers pertaining to infomration of the publishing message
     /// - Message id
     /// - Timestamp of the message sent
     /// - Message priority defined in the type [`MessagePriority`]
     /// - Message category defined in the type [`MessageCategory`]
     /// - Content format defined in the type [`ContentType`]
     Message {
          id: String,
          timestamp: Option<SystemTime>,
          priority: MessagePriority,
          category: MessageCategory,
          content_type: ContentType,
     },

     /// All information pertaining to the publishing of the message.
     /// - Queue to which the message is published
     /// - [`MessagePublish`] type.
     MessagePublish {
          queue: String,
          to: MessagePublish,
     },

     QueueCreation{
          
     }
}

/// `MTPAuth` represents different authentication methods that can be used within the protocol's header.
/// This enum defines various schemes for authentication that are relevant for managing secure communication
/// and access control in the message broker system.
///
/// Each variant represents a distinct method of authentication that can be specified within the protocol's
/// header to validate clients or manage access rights.
///
/// ## Variants
///
/// ### `ExternalToken`
///
/// Represents an external authentication token. This token is typically obtained from an external authentication
/// service or identity provider. It is used to authenticate a client by verifying the token against the external
/// service.
///
/// ### `LocalToken`
///
/// Represents a locally generated authentication token. This token is created and managed by the local authentication
/// system and is used for authenticating clients within the protocol without involving external services.
///
/// ### `Authorization`
///
/// Represents an authorization scheme used for authentication. This variant includes the following field:
/// 
/// - `scheme`: An [`AuthSchemes`] enum specifying the authentication protocol (e.g., Bearer, Basic). This field indicates
///   the type of authorization scheme being used, and the actual authorization key or token would typically accompany
///   this scheme in a real-world scenario.
///
/// ### `Cookie`
///
/// Represents authentication information contained in a cookie. This method involves the use of a cookie for
/// authentication purposes, without requiring additional parameters.
/// 
/// - The `Cookie` variant in [`MTPHeaderUnit`]'s `Authentication` field will use this variant to specify that the
///   authentication is performed using a cookie-based approach.
///
/// ## Context in [`MTPHeaderUnit`]
///
/// Within the [`MTPHeaderUnit`] enum, the `Authentication` variant is used to specify authentication-related information
/// in the protocol's header. It includes:
///
/// - `key`: An `MTPAuth` enum indicating the type of authentication used (e.g., ExternalToken, LocalToken, Authorization, or Cookie).
/// - `value`: A `String` containing the authentication data or token, which could be a token, key, or any other relevant
///   authentication information.
///
/// ## Example
///
/// Here is how `MTPAuth` might be used within the [`MTPHeaderUnit`] enum in practice:
///
/// ```rust
/// pub enum MTPHeaderUnit {
///     /// All headers pertaining to authentication of the user
///     /// Includes token from foreign security services
///     Authentication {
///         key: MTPAuth,
///         value: String,
///     },
///     // Other header units...
/// }
///
/// fn process_authentication(header_unit: MTPHeaderUnit) {
///     if let MTPHeaderUnit::Authentication { key, value } = header_unit {
///         match key {
///             MTPAuth::ExternalToken => {
///                 // Handle external token authentication
///             },
///             MTPAuth::LocalToken => {
///                 // Handle local token authentication
///             },
///             MTPAuth::Authorization { scheme } => {
///                 match scheme {
///                     AuthSchemes::Bearer => {
///                         // Handle Bearer token authentication
///                     },
///                     AuthSchemes::Basic => {
///                         // Handle Basic authentication
///                     },
///                 }
///                 // Use the value for authorization key or token
///             },
///             MTPAuth::Cookie => {
///                 // Handle cookie-based authentication
///             },
///         }
///     }
/// }
/// ```
/// 
/// In this example, `process_authentication` demonstrates how to handle various authentication methods based on the
/// `MTPAuth` variant included in the `Authentication` header unit.
pub enum MTPAuth {
     ExternalToken,
     LocalToken,
     Authorization {
          scheme: AuthSchemes
     },
     Cookie
}

/// `AuthSchemes` defines the different authentication schemes supported by the protocol.
/// This enum specifies the types of authentication schemes that can be used to authenticate
/// clients or users within the protocol, providing flexibility in how authentication is handled.
///
/// Each variant represents a distinct authentication scheme that dictates how credentials or
/// tokens are provided and validated.
///
/// ## Variants
///
/// ### `Bearer`
///
/// Represents the Bearer authentication scheme. This scheme is typically used with OAuth 2.0 tokens.
/// The Bearer scheme involves passing a token in the `Authorization` header of the HTTP request.
/// The token is used to authenticate the client or user and grant access to resources.
///
/// - **Example Usage**: `Authorization: Bearer <token>`
///
/// ### `Basic`
///
/// Represents the Basic authentication scheme. This scheme involves sending a username and password
/// encoded in Base64 in the `Authorization` header of the HTTP request. The server then decodes
/// the credentials and validates them to authenticate the client or user.
///
/// - **Example Usage**: `Authorization: Basic <base64(username:password)>`
///
/// ## Example
///
/// Here is an example of how `AuthSchemes` might be used within the protocol:
///
/// ```rust
/// pub enum AuthSchemes {
///     Bearer,
///     Basic,
/// }
///
/// fn handle_auth_scheme(scheme: AuthSchemes, credentials: &str) {
///     match scheme {
///         AuthSchemes::Bearer => {
///             // Handle Bearer token authentication
///             println!("Handling Bearer token: {}", credentials);
///         },
///         AuthSchemes::Basic => {
///             // Handle Basic authentication
///             println!("Handling Basic authentication with credentials: {}", credentials);
///         },
///     }
/// }
/// ```
/// 
/// In this example, `handle_auth_scheme` uses a `match` statement to process different authentication
/// schemes based on the `AuthSchemes` variant, applying appropriate handling for Bearer or Basic authentication.
pub enum AuthSchemes {
    Bearer,
    Basic,
}

/// `MTPManagerAction` defines the different management actions that can be performed within the protocol.
/// These actions are used to manage and control various aspects of the protocol's operation, particularly
/// in the context of user access, permissions, and resource management.
///
/// Each variant represents a specific management action that can be executed to modify or control protocol
/// behavior or resources.
///
/// ## Variants
///
/// ### `Rename`
///
/// Represents an action to rename an entity, such as a queue or resource, within the protocol. This action
/// is used to change the name of an existing entity to a new name.
///
/// - **Usage**: This action might be used when reorganizing or updating resource names in the system.
///
/// ### `Authorize`
///
/// Represents an action to authorize a user or client. This is typically used for user invites or granting
/// access to specific resources. It is a private action used to manage access rights within the protocol.
///
/// - **Usage**: This action is used to invite or grant access to a user, managing their permissions within the system.
///
/// ### `Reject`
///
/// Represents an action to reject a request or access permission. This action is used to deny access or
/// permissions to a user or client within the protocol.
///
/// - **Usage**: This action might be used to deny access requests or remove permissions from users or clients.
///
/// ### `Dispose`
///
/// Represents an action to dispose of existing clients or resources. This action is used to remove or clean up
/// clients or resources that are no longer needed or are being terminated.
///
/// - **Usage**: This action is used to remove or terminate clients or resources that are no longer active or required.
///
/// ### `AccessorModify`
///
/// Represents an action to modify the permissions or access rights of users or clients. This action allows for
/// changing the access permissions of entities within the protocol, adjusting their rights or roles.
///
/// - **Usage**: This action is used to update or change access permissions or roles for users or clients, modifying
///   their level of access to resources.
///
/// ## Example
///
/// Here is an example of how `MTPManagerAction` might be used within the protocol:
///
/// ```rust
/// pub enum MTPManagerAction {
///     Rename,
///     Authorize,
///     Reject,
///     Dispose,
///     AccessorModify,
/// }
///
/// fn perform_action(action: MTPManagerAction) {
///     match action {
///         MTPManagerAction::Rename => {
///             // Handle renaming action
///             println!("Performing rename action");
///         },
///         MTPManagerAction::Authorize => {
///             // Handle authorization action
///             println!("Performing authorization action");
///         },
///         MTPManagerAction::Reject => {
///             // Handle reject action
///             println!("Performing reject action");
///         },
///         MTPManagerAction::Dispose => {
///             // Handle dispose action
///             println!("Performing dispose action");
///         },
///         MTPManagerAction::AccessorModify => {
///             // Handle accessor modify action
///             println!("Performing accessor modify action");
///         },
///     }
/// }
/// ```
/// 
/// In this example, the `perform_action` function uses a `match` statement to handle different management actions
/// based on the `MTPManagerAction` variant, performing appropriate operations for each action.
pub enum MTPManagerAction {
     /// Rename an existing resource specifically the queue in the which the moderator is operating
     Rename(String),

     /// Authorize user requesting persmissions to the queue
     Authorize(String),       // On user invites (private)

     /// Reject user requesting permissions to the queue
     Reject,          // Access permission

     /// Dispose an exising client from the queue
     Dispose(String),         // Dispose of existing clients

     /// Modify the roles/permissions of existing client
     AccessorModify(QueueAccess),  // Change the permission of the access of the queue
}

/// [`QueueAccess`] defines an access of a client to a particular queue.
/// 
/// ## Variants
///
/// ### `Public`
///
/// A queue to be public. Anyone with the queue identifier can join the queue to receive messages
/// This is the default queue accessor.
///
/// - **Usage**: A public queue to publish messages to all who join the queue wihtout needing to wait for authorization
///
/// /// ### `Private`
///
/// A queue to be private can only be joined if the admin authorizes the request.
/// A client waits for admin aprroval and then joins the queue to receive published messages
///
/// - **Usage**: A private queue to restrict information acccess
///
/// ### `Protected`
///
/// A queue to be protected.
///
pub enum QueueAccess {
    Public,
    Private,
    Protected
}

/// `MessagePriority` defines the priority levels of messages within the protocol.
/// This enum specifies how messages are prioritized when being processed or transmitted,
/// allowing the system to handle messages according to their importance or urgency.
///
/// Each variant represents a different priority level that can be assigned to messages,
/// influencing their order of processing and delivery.
///
/// ## Variants
///
/// ### `Low`
///
/// Represents a low priority message. Messages with this priority are considered less urgent
/// and may be processed after higher-priority messages. They are typically used for non-critical
/// information or routine updates that do not require immediate attention.
///
/// - **Usage**: Use this priority for messages that can tolerate delays and are not time-sensitive.
///
/// ### `Medium`
///
/// Represents a medium priority message. Messages with this priority are more important than low-priority
/// messages but less urgent than high or critical messages. They are suitable for important information
/// that should be processed in a timely manner, though not as urgently as higher-priority messages.
///
/// - **Usage**: Use this priority for messages that need timely processing but are not critical.
///
/// ### `High`
///
/// Represents a high priority message. Messages with this priority are considered urgent and should
/// be processed before lower-priority messages. They are typically used for important alerts or actions
/// that need prompt attention.
///
/// - **Usage**: Use this priority for messages that require quick processing due to their importance.
///
/// ### `Critical`
///
/// Represents a critical priority message. Messages with this priority are of the highest urgency and
/// should be processed immediately. They are used for critical alerts or actions that require immediate
/// attention and resolution.
///
/// - **Usage**: Use this priority for messages that demand immediate processing and cannot be delayed.
///
/// ## Example
///
/// Here is an example of how `MessagePriority` might be used within the protocol:
///
/// ```rust
/// pub enum MessagePriority {
///     Low,
///     Medium,
///     High,
///     Critical,
/// }
///
/// fn process_message(priority: MessagePriority) {
///     match priority {
///         MessagePriority::Low => {
///             // Handle low priority message
///             println!("Processing low priority message");
///         },
///         MessagePriority::Medium => {
///             // Handle medium priority message
///             println!("Processing medium priority message");
///         },
///         MessagePriority::High => {
///             // Handle high priority message
///             println!("Processing high priority message");
///         },
///         MessagePriority::Critical => {
///             // Handle critical priority message
///             println!("Processing critical priority message");
///         },
///     }
/// }
/// ```
/// 
/// In this example, the `process_message` function uses a `match` statement to handle different message
/// priority levels, performing specific operations based on the priority assigned to each message.
pub enum MessagePriority {
     /// Low priority messages
     Low,

     /// Mediumm priority messages
     Medium,

     /// High priority messages
     High,

     /// Critical messsages (highest priority in the queue)
     Critical,
}

/// `MessageCategory` defines the different categories that a [`MTPMessage`] can belong to.
/// This enum categorizes messages based on their purpose or type, helping to organize and manage messages
/// according to their functional role or content.
/// 
/// This proprty is only significant to client-client interactions and not with client-server interactions.
///
/// Each variant represents a specific category that indicates the nature or intent of the message.
///
/// ## Variants
///
/// ### `EVENT`
///
/// Represents a message that denotes an occurrence or change of state within the system. Event messages
/// are typically used to inform other components about significant changes or actions that have taken place.
///
/// - **Usage**: Use this category for messages that are triggered by events, such as system notifications
///   or status updates.
///
/// ### `COMMAND`
///
/// Represents a message that instructs or requests an action to be performed. Command messages are used
/// to direct the system or other components to execute specific operations or tasks.
///
/// - **Usage**: Use this category for messages that carry commands or instructions, such as control commands
///   or operational directives.
///
/// ### `REQUEST`
///
/// Represents a message that requests information or a specific service from the system or other components.
/// Request messages are used to ask for data or perform an operation and typically expect a response.
///
/// - **Usage**: Use this category for messages that are sent to request information or actions, such as queries
///   or service requests.
///
/// ### `RESPONSE`
///
/// Represents a message that provides an answer or feedback in response to a request. Response messages
/// are used to deliver results or acknowledgments following a request message.
///
/// - **Usage**: Use this category for messages that deliver responses or results to previous requests, such
///   as replies to queries or status updates.
///
/// ### `ACKNOWLEDGEMENT`
///
/// Represents a message that confirms receipt of another message or action. Acknowledgment messages
/// are used to signal that a message has been received and processed successfully.
///
/// - **Usage**: Use this category for messages that serve as confirmations or acknowledgments, such as receipt
///   acknowledgments or confirmation of actions.
///
/// ### `ERROR`
///
/// Represents a message that indicates an error or problem within the system. Error messages are used to
/// report issues or failures that need attention or resolution.
///
/// - **Usage**: Use this category for messages that convey error conditions or exceptions, such as error reports
///   or failure notifications.
///
/// ### `NOTIFICATION`
///
/// Represents a message that provides information or alerts without requiring any action or response.
/// Notification messages are used to deliver updates or general information to other components or users.
///
/// - **Usage**: Use this category for messages that serve as notifications or alerts, such as system updates
///   or general information broadcasts.
///
/// ### `STATUS`
///
/// Represents a message that conveys the current state or status of a system or component. Status messages
/// are used to provide periodic or on-demand status information about the system's condition.
///
/// - **Usage**: Use this category for messages that provide status information or system health reports, such
///   as status updates or health checks.
///
/// ## Example
///
/// Here is an example of how `MessageCategory` might be used within the protocol:
///
/// ```rust
/// pub enum MessageCategory {
///     EVENT,
///     COMMAND,
///     REQUEST,
///     RESPONSE,
///     ACKNOWLEDGEMENT,
///     ERROR,
///     NOTIFICATION,
///     STATUS,
/// }
///
/// fn handle_message(category: MessageCategory) {
///     match category {
///         MessageCategory::EVENT => {
///             // Handle event message
///             println!("Handling event message");
///         },
///         MessageCategory::COMMAND => {
///             // Handle command message
///             println!("Handling command message");
///         },
///         MessageCategory::REQUEST => {
///             // Handle request message
///             println!("Handling request message");
///         },
///         MessageCategory::RESPONSE => {
///             // Handle response message
///             println!("Handling response message");
///         },
///         MessageCategory::ACKNOWLEDGEMENT => {
///             // Handle acknowledgment message
///             println!("Handling acknowledgment message");
///         },
///         MessageCategory::ERROR => {
///             // Handle error message
///             println!("Handling error message");
///         },
///         MessageCategory::NOTIFICATION => {
///             // Handle notification message
///             println!("Handling notification message");
///         },
///         MessageCategory::STATUS => {
///             // Handle status message
///             println!("Handling status message");
///         },
///     }
/// }
/// ```
/// 
/// In this example, the `handle_message` function uses a `match` statement to handle different message
/// categories, performing specific operations based on the category assigned to each message.

pub enum MessageCategory {
    EVENT,
    COMMAND,
    REQUEST,
    RESPONSE,
    ACKNOWLEDGEMENT,
    ERROR,
    NOTIFICATION,
    STATUS,
}

/// `ContentType` defines the content types supported by the protocol.
pub enum ContentType {
    JSON,
    XML,
}

/// [`QueueRoles`] for the queue
/// Roles that are defined for each client in the queue
pub enum QueueRoles {
    Moderator,
    Manager,
    Producer,
    Consumer,
    Couple
}



/// `MessagePublish` defines the different ways a message can be published within the protocol.
/// This enum specifies the different targets or scopes to which a message can be sent,
/// allowing for flexible and varied message dissemination strategies.
///
/// Each variant represents a distinct method for publishing a message, determining how
/// and to whom the message will be delivered.
///
/// ## Variants
///
/// ### `ALL`
///
/// Represents publishing the message to all available recipients or subscribers.
/// When using this method, the message is broadcasted to every possible recipient
/// within the system, ensuring that all parties receive the message.
///
/// - **Usage**: Use this variant when the message needs to be delivered universally
///   to all subscribers or recipients.
///
/// ### `TO`
///
/// Represents publishing the message to a specific recipient or queue.
/// This variant allows targeting a single recipient by specifying a unique identifier
/// or address to which the message should be delivered.
///
/// - **Usage**: Use this variant when the message is intended for a particular recipient
///   or queue, requiring direct and targeted delivery.
///
/// ### `GROUP`
///
/// Represents publishing the message to a group of recipients, where the group is specified
/// by a vector of identifiers or addresses. This allows sending the message to multiple
/// recipients as defined by the group.
///
/// - **Type**: `Vec<String>`
/// - **Usage**: Use this variant when the message should be delivered to a predefined group
///   of recipients. The vector contains the identifiers or addresses of the group members.
///
/// ## Example
///
/// Here is an example of how `MessagePublish` might be used:
///
/// ```rust
/// pub enum MessagePublish {
///     ALL,
///     TO(String),
///     GROUP(Vec<String>),
/// }
///
/// fn publish_message(publish_method: MessagePublish, message: &str) {
///     match publish_method {
///         MessagePublish::ALL => {
///             // Publish message to all recipients
///             println!("Publishing message to all recipients: {}", message);
///         },
///         MessagePublish::TO(recipient) => {
///             // Publish message to a specific recipient
///             println!("Publishing message to recipient {}: {}", recipient, message);
///         },
///         MessagePublish::GROUP(recipients) => {
///             // Publish message to a group of recipients
///             for recipient in recipients {
///                 println!("Publishing message to group recipient {}: {}", recipient, message);
///             }
///         },
///     }
/// }
/// ```
///
/// In this example, the `publish_message` function demonstrates how to handle different
/// publishing methods based on the `MessagePublish` variant. It shows how to publish messages
/// to all recipients, a specific recipient, or a group of recipients.

pub enum MessagePublish {

     /// Default all clients registered in the queue
     ALL,

     // To a specific client in the queue
     TO(String),

     /// passes the identifier of a client in the list and publish to specified client
     GROUP(Vec<String>),
}


/// Clone implementation for [MTPAuth]
impl Clone for MTPAuth{
     fn clone(&self) -> Self {
         match self {
             Self::ExternalToken => Self::ExternalToken,
             Self::LocalToken => Self::LocalToken,
             Self::Authorization { scheme } => Self::Authorization { scheme: scheme.clone() },
             Self::Cookie => Self::Cookie,
         }
     }
}

/// Clone implementation for [AuthSchemes]
impl Clone for AuthSchemes {
     fn clone(&self) -> Self {
          match self {
             Self::Bearer => Self::Bearer,
             Self::Basic => Self::Basic,
          }
     }
}

/// Clone implementation for [MessagePriority]
impl Clone for MessagePriority{
    fn clone(&self) -> Self {
        match self {
            Self::Low => Self::Low,
            Self::Medium => Self::Medium,
            Self::High => Self::High,
            Self::Critical => Self::Critical,
        }
    }
}

/// Clone implementation for [MessageCategory]
impl Clone for MessageCategory{
     fn clone(&self) -> Self {
          match self {
               Self::EVENT => Self::EVENT,
               Self::COMMAND => Self::COMMAND,
               Self::REQUEST => Self::REQUEST,
               Self::RESPONSE => Self::RESPONSE,
               Self::ACKNOWLEDGEMENT => Self::ACKNOWLEDGEMENT,
               Self::ERROR => Self::ERROR,
               Self::NOTIFICATION => Self::NOTIFICATION,
               Self::STATUS => Self::STATUS,
          }
     }
}


/// Clone implementation for [ContentType]
impl Clone for ContentType{
     fn clone(&self) -> Self {
          match self {
               Self::JSON => Self::JSON,
               Self::XML => Self::XML,
          }
     }
}

/// Clone implementation for [MessagePublish]
impl Clone for MessagePublish{
     fn clone(&self) -> Self {
         match self {
             Self::ALL => Self::ALL,
             Self::TO(arg0) => Self::TO(arg0.clone()),
             Self::GROUP(arg0) => Self::GROUP(arg0.clone()),
         }
     }
 }

/// Clone implementation for [MTPRequestType]
impl Clone for MTPRequestType{
     fn clone(&self) -> Self {
          match self {
               Self::Subscribe => Self::Subscribe,
               Self::Unsubscribe => Self::Unsubscribe,
               Self::Publish => Self::Publish,
               Self::Pull => Self::Pull,
               Self::Ping => Self::Ping,
               Self::Manage => Self::Manage,
          }
     }
}

/// Clone implementation for [QueueAccess]
impl Clone for QueueAccess{
    fn clone(&self) -> Self {
        match self {
            Self::Public => Self::Public,
            Self::Private => Self::Private,
            Self::Protected => Self::Protected,
        }
    }
}