
pub mod error;
pub mod interface;

use net::socket::server::data::SocketData;
use crate::security::manager::SecurityManager;

use super::preface_middleware::Middleware;

pub struct ServerPrologue<S>
where S:SecurityManager{
     security:S,
     middlewares:Vec<Box<dyn Middleware>>
}

impl <T:SecurityManager>ServerPrologue<T> {
     pub fn new(security:T)->Self
     where T:SecurityManager{
          //middlewares
          let middlewares = Vec::new();

          Self{
               security,
               middlewares
          }
     }

     pub fn config_middleware(&mut self, middleware:T)->&mut Self
     where T:Middleware + 'static{
          self.middlewares.push(Box::new(middleware));
          return self;
     }

     pub fn add_middleware(&mut self, middleware:T)
     where T:Middleware + 'static{
          self.middlewares.push(Box::new(middleware));
     }

     async fn run_middlewares(&self, socket_data:SocketData)->SocketData{
          let mut socket_data_transfered = socket_data;

          // Runs each middleware configured in for this ServerPrologue
          for middleware in &self.middlewares{
               socket_data_transfered =  middleware.parse(socket_data_transfered);
          }

          socket_data_transfered
     }

     async fn run_prologue(&mut self, socket_data:SocketData){

     }

     async fn secure(&mut self, socket_data:SocketData){

     }


}
