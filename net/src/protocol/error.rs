/// Enum representing various errors in the Message Transfer Protocol (MTP).
/// These errors are categorized into Client Errors (100-115) and Server Errors (120-128).
/// These errors are also sent along with Response
/// These errors indicate issues on the client side of the protocol.
 
/// - 100 - Bad Request: The request could not be understood or was missing required parameters.
/// BadRequest100(Error),
 
/// - 101 - Unauthorized: Authentication is required and has failed or has not been provided.
/// Unauthorized101(Error),
 
/// - 102 - Forbidden: The request is understood, but it has been refused or access is not allowed.
/// Forbidden102(Error),
 
/// - 103 - Not Found: The requested resource could not be found.
/// NotFound103(Error),
 
/// - 104 - Method Not Allowed: The method specified in the request is not allowed for the resource.
/// MethodNotAllowed104(Error),
 
/// - 105 - Not Acceptable: The resource is capable of generating only content not acceptable according to the Accept headers sent in the request.
/// NotAcceptable105(Error),
 
/// - 106 - Proxy Authentication Required: Authentication with a proxy is required.
/// ProxyAuthenticationRequired106(Error),
 
/// - 107 - Request Timeout: The server timed out waiting for the request.
/// RequestTimeout107(Error),
 
/// - 108 - Conflict: The request could not be processed because of conflict in the current state of the resource.
/// Conflict108(Error),
 
/// - 109 - Gone: The requested resource is no longer available and will not be available again.
/// Gone109(Error),
 
/// - 110 - Precondition Failed: The server does not meet one of the preconditions that the requester put on the request.
/// PreconditionFailed110(Error),
 
/// - 111 - Payload Too Large: The request is larger than the server is willing or able to process.
/// PayloadTooLarge111(Error),
 
/// - 112 - Unprocessable Content: The server understands the content type of the request entity, but was unable to process the contained instructions.
/// UnprocessableContent112(Error),
 
/// - 113 - Locked: The resource is currently locked and cannot be accessed.
/// Locked113(Error),
 
/// - 114 - Too Many Requests: The user has sent too many requests in a given amount of time.
/// TooManyRequests114(Error),

/// - 115 - Request Header Too Large: The request headers are too large for the server to process.
/// RequestHeaderTooLarge115(Error),

/// These errors indicate issues on the server side of the protocol.
///  - 120 - Internal Server Error: An unexpected condition was encountered on the server.
/// InternalServerError120(Error),
 
///  - 121 - Bad Gateway: The server received an invalid response from the upstream server.
/// BadGateway121(Error),
 
///  - 123 - Service Unavailable: The server is currently unable to handle the request due to a temporary overload or maintenance.
/// ServiceUnavailable123(Error),
 
///  - 124 - Gateway Timeout: The server did not receive a timely response from the upstream server or some other auxiliary server.
/// GatewayTimeout124(Error),
 
///  - 125 - MTP Version Not Supported: The MTP version used in the request is not supported by the server.
/// MTPVersionNotSupported125(Error),
 
///  - 126 - Insufficient Storage: The server is unable to store the representation needed to complete the request.
/// InsufficientStorage126(Error),
 
///  - 127 - Loop Detected: The server detected an infinite loop while processing the request.
/// LoopDetected127(Error),
 
///  - 128 - Network Authentication Required: The request requires network authentication.
/// NetworkAuthenticationRequired128(Error),
pub enum ProtocolError {

     /// **Client Errors (100-115)**
     ///
     /// These errors indicate issues on the client side of the protocol.
 
     /// 100 - Bad Request: The request could not be understood or was missing required parameters.
     BadRequest100(Error),
 
     /// 101 - Unauthorized: Authentication is required and has failed or has not been provided.
     Unauthorized101(Error),
 
     /// 102 - Forbidden: The request is understood, but it has been refused or access is not allowed.
     Forbidden102(Error),
 
     /// 103 - Not Found: The requested resource could not be found.
     NotFound103(Error),
 
     /// 104 - Method Not Allowed: The method specified in the request is not allowed for the resource.
     MethodNotAllowed104(Error),
 
     /// 105 - Not Acceptable: The resource is capable of generating only content not acceptable according to the Accept headers sent in the request.
     NotAcceptable105(Error),
 
     /// 106 - Proxy Authentication Required: Authentication with a proxy is required.
     ProxyAuthenticationRequired106(Error),
 
     /// 107 - Request Timeout: The server timed out waiting for the request.
     RequestTimeout107(Error),
 
     /// 108 - Conflict: The request could not be processed because of conflict in the current state of the resource.
     Conflict108(Error),
 
     /// 109 - Gone: The requested resource is no longer available and will not be available again.
     Gone109(Error),
 
     /// 110 - Precondition Failed: The server does not meet one of the preconditions that the requester put on the request.
     PreconditionFailed110(Error),
 
     /// 111 - Payload Too Large: The request is larger than the server is willing or able to process.
     PayloadTooLarge111(Error),
 
     /// 112 - Unprocessable Content: The server understands the content type of the request entity, but was unable to process the contained instructions.
     UnprocessableContent112(Error),
 
     /// 113 - Locked: The resource is currently locked and cannot be accessed.
     Locked113(Error),
 
     /// 114 - Too Many Requests: The user has sent too many requests in a given amount of time.
     TooManyRequests114(Error),
 
     /// 115 - Request Header Too Large: The request headers are too large for the server to process.
     RequestHeaderTooLarge115(Error),
 
 
     /// **Server Errors (120-128)**
     ///
     /// These errors indicate issues on the server side of the protocol.
 
     /// 120 - Internal Server Error: An unexpected condition was encountered on the server.
     InternalServerError120(Error),
 
     /// 121 - Bad Gateway: The server received an invalid response from the upstream server.
     BadGateway121(Error),
 
     /// 123 - Service Unavailable: The server is currently unable to handle the request due to a temporary overload or maintenance.
     ServiceUnavailable123(Error),
 
     /// 124 - Gateway Timeout: The server did not receive a timely response from the upstream server or some other auxiliary server.
     GatewayTimeout124(Error),
 
     /// 125 - MTP Version Not Supported: The MTP version used in the request is not supported by the server.
     MTPVersionNotSupported125(Error),
 
     /// 126 - Insufficient Storage: The server is unable to store the representation needed to complete the request.
     InsufficientStorage126(Error),
 
     /// 127 - Loop Detected: The server detected an infinite loop while processing the request.
     LoopDetected127(Error),
 
     /// 128 - Network Authentication Required: The request requires network authentication.
     NetworkAuthenticationRequired128(Error),
 }
 

/// Error type for a Protocol error
/// Handles additional information about the error that occured and is sent to the client 
pub struct Error{
     info:String
}

impl ProtocolError {
     /// Returns the numeric code associated with the error.
     ///
     /// # Examples
     ///
     /// ```
     /// let error = ProtocolError::BadRequest100(Error { message: "Invalid request".to_string() });
     /// assert_eq!(error.code(), 100);
     /// ```
     fn code(&self) -> u32 {
          match self {
             ProtocolError::BadRequest100(_) => 100,
             ProtocolError::Unauthorized101(_) => 101,
             ProtocolError::Forbidden102(_) => 102,
             ProtocolError::NotFound103(_) => 103,
             ProtocolError::MethodNotAllowed104(_) => 104,
             ProtocolError::NotAcceptable105(_) => 105,
             ProtocolError::ProxyAuthenticationRequired106(_) => 106,
             ProtocolError::RequestTimeout107(_) => 107,
             ProtocolError::Conflict108(_) => 108,
             ProtocolError::Gone109(_) => 109,
             ProtocolError::PreconditionFailed110(_) => 110,
             ProtocolError::PayloadTooLarge111(_) => 111,
             ProtocolError::UnprocessableContent112(_) => 112,
             ProtocolError::Locked113(_) => 113,
             ProtocolError::TooManyRequests114(_) => 114,
             ProtocolError::RequestHeaderTooLarge115(_) => 115,
             ProtocolError::InternalServerError120(_) => 120,
             ProtocolError::BadGateway121(_) => 121,
             ProtocolError::ServiceUnavailable123(_) => 123,
             ProtocolError::GatewayTimeout124(_) => 124,
             ProtocolError::MTPVersionNotSupported125(_) => 125,
             ProtocolError::InsufficientStorage126(_) => 126,
             ProtocolError::LoopDetected127(_) => 127,
             ProtocolError::NetworkAuthenticationRequired128(_) => 128,
         }
     }
 
     /// Returns a description of the error.
     ///
     /// # Examples
     ///
     /// ```
     /// let error = ProtocolError::BadRequest100(Error { message: "Invalid request".to_string() });
     /// assert_eq!(error.description(), "100 - Bad Request: The request could not be understood or was missing required parameters.");
     /// ```
     fn description(&self) -> &'static str {
          match self {
               ProtocolError::BadRequest100(_) => "100 - Bad Request: The request could not be understood or was missing required parameters.",
               ProtocolError::Unauthorized101(_) => "101 - Unauthorized: Authentication is required and has failed or has not been provided.",
               ProtocolError::Forbidden102(_) => "102 - Forbidden: The request is understood, but it has been refused or access is not allowed.",
               ProtocolError::NotFound103(_) => "103 - Not Found: The requested resource could not be found.",
               ProtocolError::MethodNotAllowed104(_) => "104 - Method Not Allowed: The method specified in the request is not allowed for the resource.",
               ProtocolError::NotAcceptable105(_) => "105 - Not Acceptable: The resource is capable of generating only content not acceptable according to the Accept headers sent in the request.",
               ProtocolError::ProxyAuthenticationRequired106(_) => "106 - Proxy Authentication Required: Authentication with a proxy is required.",
               ProtocolError::RequestTimeout107(_) => "107 - Request Timeout: The server timed out waiting for the request.",
               ProtocolError::Conflict108(_) => "108 - Conflict: The request could not be processed because of conflict in the current state of the resource.",
               ProtocolError::Gone109(_) => "109 - Gone: The requested resource is no longer available and will not be available again.",
               ProtocolError::PreconditionFailed110(_) => "110 - Precondition Failed: The server does not meet one of the preconditions that the requester put on the request.",
               ProtocolError::PayloadTooLarge111(_) => "111 - Payload Too Large: The request is larger than the server is willing or able to process.",
               ProtocolError::UnprocessableContent112(_) => "112 - Unprocessable Content: The server understands the content type of the request entity, but was unable to process the contained instructions.",
               ProtocolError::Locked113(_) => "113 - Locked: The resource is currently locked and cannot be accessed.",
               ProtocolError::TooManyRequests114(_) => "114 - Too Many Requests: The user has sent too many requests in a given amount of time.",
               ProtocolError::RequestHeaderTooLarge115(_) => "115 - Request Header Too Large: The request headers are too large for the server to process.",
               ProtocolError::InternalServerError120(_) => "120 - Internal Server Error: An unexpected condition was encountered on the server.",
               ProtocolError::BadGateway121(_) => "121 - Bad Gateway: The server received an invalid response from the upstream server.",
               ProtocolError::ServiceUnavailable123(_) => "123 - Service Unavailable: The server is currently unable to handle the request due to a temporary overload or maintenance.",
               ProtocolError::GatewayTimeout124(_) => "124 - Gateway Timeout: The server did not receive a timely response from the upstream server or some other auxiliary server.",
               ProtocolError::MTPVersionNotSupported125(_) => "125 - MTP Version Not Supported: The MTP version used in the request is not supported by the server.",
               ProtocolError::InsufficientStorage126(_) => "126 - Insufficient Storage: The server is unable to store the representation needed to complete the request.",
               ProtocolError::LoopDetected127(_) => "127 - Loop Detected: The server detected an infinite loop while processing the request.",
               ProtocolError::NetworkAuthenticationRequired128(_) => "128 - Network Authentication Required: The request requires network authentication.",
          }
     }
 }

/// Clone implementation for [`ProtocolError`]
impl Clone for ProtocolError{
     fn clone(&self) -> Self {
          match self {
               Self::BadRequest100(arg0) => Self::BadRequest100(arg0.clone()),
               Self::Unauthorized101(arg0) => Self::Unauthorized101(arg0.clone()),
               Self::Forbidden102(arg0) => Self::Forbidden102(arg0.clone()),
               Self::NotFound103(arg0) => Self::NotFound103(arg0.clone()),
               Self::MethodNotAllowed104(arg0) => Self::MethodNotAllowed104(arg0.clone()),
               Self::NotAcceptable105(arg0) => Self::NotAcceptable105(arg0.clone()),
               Self::ProxyAuthenticationRequired106(arg0) => Self::ProxyAuthenticationRequired106(arg0.clone()),
               Self::RequestTimeout107(arg0) => Self::RequestTimeout107(arg0.clone()),
               Self::Conflict108(arg0) => Self::Conflict108(arg0.clone()),
               Self::Gone109(arg0) => Self::Gone109(arg0.clone()),
               Self::PreconditionFailed110(arg0) => Self::PreconditionFailed110(arg0.clone()),
               Self::PayloadTooLarge111(arg0) => Self::PayloadTooLarge111(arg0.clone()),
               Self::UnprocessableContent112(arg0) => Self::UnprocessableContent112(arg0.clone()),
               Self::Locked113(arg0) => Self::Locked113(arg0.clone()),
               Self::TooManyRequests114(arg0) => Self::TooManyRequests114(arg0.clone()),
               Self::RequestHeaderTooLarge115(arg0) => Self::RequestHeaderTooLarge115(arg0.clone()),
               Self::InternalServerError120(arg0) => Self::InternalServerError120(arg0.clone()),
               Self::BadGateway121(arg0) => Self::BadGateway121(arg0.clone()),
               Self::ServiceUnavailable123(arg0) => Self::ServiceUnavailable123(arg0.clone()),
               Self::GatewayTimeout124(arg0) => Self::GatewayTimeout124(arg0.clone()),
               Self::MTPVersionNotSupported125(arg0) => Self::MTPVersionNotSupported125(arg0.clone()),
               Self::InsufficientStorage126(arg0) => Self::InsufficientStorage126(arg0.clone()),
               Self::LoopDetected127(arg0) => Self::LoopDetected127(arg0.clone()),
               Self::NetworkAuthenticationRequired128(arg0) => Self::NetworkAuthenticationRequired128(arg0.clone()),
          }
     }
}

/// Clone implementation for [Error]
impl Clone for Error{
    fn clone(&self) -> Self {
        Self { info: self.info.clone() }
    }
}