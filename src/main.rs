use std::collections::HashMap;
use eframe::egui;
use niri_ipc::socket::{Socket, SOCKET_PATH_ENV};
use niri_ipc::{ConfiguredPosition, Output, OutputAction, Request, Response};

// connect to a socket
    // send a [Request] -> [Reply] ( Result(Response))
    // reply is a JSON formatted;
    //
    //
    // pub enum PositionToSet; and define a ConfiguredPosition
    //
    //
    // recover the current config - talk with the ipc
    //


fn get_outputs(socket : &mut Socket) -> HashMap<String, Output>{

  let request = Request::Outputs;
  let reply = socket.send(request);

  let outputs  = match reply{
    Ok(Ok(Response::Outputs(outputs))) =>{
      outputs
    },
    Err(_) =>{
      HashMap::new()
    },
    Ok(_) =>{
      HashMap::new()
    },
  };

  outputs

}

pub struct Manager{
  socket : Socket,
  outputs : HashMap<String, Output>,
}

impl Manager{
  pub fn new() -> Manager{
    let s = Socket::connect();
    let mut socket = match s{
      Ok(sock) => sock,
      Err(_) => {
        panic!("Couldn't establish a connection with the Niri IPC!")
      }
    };

    let outputs = get_outputs(&mut socket);

    Manager{
      socket : socket,
      outputs : outputs
    }
  }

  pub fn change_layout(&mut self, output : &String, x : i32, y : i32){
    // so apparently niri does coordinate space with a y that does down
    let config = ConfiguredPosition{ x: x , y:y};
      let action = OutputAction::Position {
        position: niri_ipc::PositionToSet::Specific(config),
      };

      let request = Request::Output{
        output : output.clone(),
        action : action,
      };

      let res = self.socket.send(request);

      match res {
          Ok(_) =>{
            println!("Changes done successfully")
          },
          Err(_) => {
            println!("Error");
          }
      }
  }

  pub fn show_outputs(&self){

    let mut i = 0;
    for (name, o ) in &self.outputs{
      if let Some(l) = &o.logical{
        println!("=============");
        println!("Output {} {}",i ,o.name);
        println!("{}", l.x);
        println!("{}", l.y);
        println!("{}", l.width);
        println!("{}", l.height);
        println!("=============");
        i+=1;
      }
    }
  }

}

fn main(){
  // let mut socket = Socket::connect()?;
  // let outputs_res = get_outputs(&mut socket);

  let mut manager = Manager::new();

  // once we get the response from the ipc, we have to store it, we would also have to react when the user adds disconnects/adds a new monitor (future thign)
  //
  manager.show_outputs();

  manager.change_layout( &"HDMI-A-1".to_string(), 0, -1080);
  manager.change_layout( &"eDP-1".to_string(), 0, 0);

  manager.show_outputs();

  // run_native(app, native_options, app_creator)

}
