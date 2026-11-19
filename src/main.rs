use std::collections::HashMap;
use std::io;

use eframe::egui::output;
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

// so apparently niri does coordinate space with a y that does down
fn change_layout(socket :  &mut Socket,output : &String, x: i32, y : i32){

  let config = ConfiguredPosition{ x: x , y:y};
  let action = OutputAction::Position {
    position: niri_ipc::PositionToSet::Specific(config),
  };

  let request = Request::Output{
    output : output.clone(),
    action : action,
  };

  let res = socket.send(request);

  match res {
      Ok(_) =>{
        println!("Changed done successfully")
      },
      Err(_) => {
        println!("Error");
      }
  }
}

fn get_outputs(socket : &mut Socket) -> Result<HashMap<String, Output>, io::Error>{

  let request = Request::Outputs;
  let reply = socket.send(request)?;

  match reply{
    Ok(Response::Outputs(outputs)) =>{
      Ok(outputs)
    },
    Err(err) =>{
      Err(io::Error::other(err))
    },
    Ok(_) =>{Err(io::Error::other("Invalid Request"))},
  }

}
fn main() -> std::io::Result<()>{
  let mut socket = Socket::connect()?;

  let request = Request::Outputs;
  let reply = socket.send(request)?;

  let mut i = 0;
  // once we get the response from the ipc, we have to store it, we would also have to react when the user adds disconnects/adds a new monitor (future thign)
  if let Ok(Response::Outputs(outputs)) = reply{
    let outputs_cache = outputs.clone();
    for (name,o) in &outputs{
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

  change_layout(&mut socket, &"HDMI-A-1".to_string(), 0, -1080);
  change_layout(&mut socket, &"eDP-1".to_string(), 0, 0);
  Ok(())
}
