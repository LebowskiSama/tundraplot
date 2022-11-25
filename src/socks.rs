use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use egui::plot::Value;

use super::egui_app;
use egui_app::App;

use tungstenite;
use url;

macro_rules! sanitize_parse {
    ($s: expr) => {
        $s.chars().filter(|c| c.is_digit(10)).collect::<String>().to_owned().parse::<f64>().unwrap()
    };
}

pub fn receive_data(server_addr: &str, gui_app: &mut App) {
    // connect to WS server
    let (mut socket, _response) = tungstenite::connect(
        url::Url::parse(&server_addr)
        .unwrap()
    ).expect("Can't connect");

    let app_state_idx = Arc::clone(&mut gui_app.idx);
    let app_state_x = Arc::clone(&mut gui_app.buffer_x);

    // loop forever
    std::thread::spawn( 
        move || {
            loop {

                // write empty string to keep connection alive
                socket.write_message(tungstenite::Message::Text("".into())).unwrap();
        
                let msg = socket.read_message().expect("Error handling message");
                let msg = match msg {
                    tungstenite::Message::Text(s) => { s }
                    _ => { panic!() }
                };
                
                let sample = msg.to_string().split(",").map(|x| x.to_string()).collect::<Vec<String>>();

                let t = sanitize_parse!(sample[1]);
                // let x = sanitize_parse!(sample[2]);
                let x = sanitize_parse!(sample[3]);
                // let z = sanitize_parse!(sample[4]);
                
                let mut arc_app_state_idx = app_state_idx.lock().unwrap();
                let mut arc_app_state_x = app_state_x.lock().unwrap();
            
                *arc_app_state_idx += 1.0;


                arc_app_state_x.push_back(
                    Value { x: arc_app_state_idx.clone(), y: x }
                );

                // if arc_app_state_x.len() == 100 {
                //     arc_app_state_x.pop_front();
                // }
            }
        }
    );
}

