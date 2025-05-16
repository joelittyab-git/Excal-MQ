use net::protocol::error::ProtocolError;

pub enum MiddlewareError {
     InvalidData{
          info:String
     },

     ProtocolError{
          source:ProtocolError
     },

     CError{
          info:String
     }
}

impl MiddlewareError {
     pub fn new(info:String)->Self{
          Self::CError { info }
     }
}