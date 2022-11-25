use std::sync::{Arc, Mutex};
use eframe::NativeOptions;
use egui::{CtxRef, plot::{Plot, Line, Values, Value}};


mod socks;
mod egui_app;

use egui_app::App;


fn main() {
    let server_addr: String = String::from("ws://192.168.0.118:9090");
    let mut app: App = App::new();
    
    // receive and push data into GUI state
    socks::receive_data(&server_addr, &mut app);
    
    let native_options = NativeOptions::default();
    eframe::run_native(Box::new(app), native_options)

}

