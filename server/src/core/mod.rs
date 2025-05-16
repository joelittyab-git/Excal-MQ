use crate::preface::prologue::{error::PrologueError, interface::Prologue};

struct CoreServer<P:Prologue>{
     prologue:P,
}

impl <P:Prologue>CoreServer<P> {
     fn new(prologue:P)->Self
     where P:Prologue{
          Self{
               prologue
          }
     }

     fn start(&self){
          self.prologue.open_socket().inspect_err(|err|{
               println!("Something went wrong {err}");
          });
     }
}