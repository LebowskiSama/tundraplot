use std::thread;
use std::sync::{Arc, Mutex};

use tungstenite;
use url;

pub fn print_hello() {
    println!("Hello World!")
}

pub fn receive_data(server_addr: &str, data_store: &mut Arc<Mutex<Vec<Vec<String>>>>) {
    // connect to WS server
    let (mut socket, _response) = tungstenite::connect(
        url::Url::parse(&server_addr)
        .unwrap()
    ).expect("Can't connect");

    let arc_store = Arc::clone(data_store);

    // loop forever
    thread::spawn( 
        move || {
            loop {

                // write empty string to keep connection alive
                socket.write_message(tungstenite::Message::Text("".into())).unwrap();
        
                let msg = socket.read_message().expect("Error handling message");
                let msg = match msg {
                    tungstenite::Message::Text(s) => { s }
                    _ => { panic!() }
                };
                
                let msg_split = msg.to_string().split(",").map(|x| x.to_string()).collect::<Vec<String>>();
        
                
                arc_store.lock().unwrap().push(msg_split);
            }
        }
    );
}

