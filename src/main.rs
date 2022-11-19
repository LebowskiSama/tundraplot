use std::sync::{Arc, Mutex};

mod socks;
mod utils;
mod dtypes;

use dtypes::dataframes::AccDataFrame;
use utils::parser;


fn main() {
    let server_addr: String = String::from("ws://192.168.1.9:9090");
    let mut store: Arc<Mutex<Vec<Vec<String>>>> = Arc::new(Mutex::new(Vec::new()));
    let mut df: Arc<Mutex<AccDataFrame>> = Arc::new(Mutex::new(AccDataFrame::new()));

    socks::receive_data(&server_addr, &mut store);
    
    loop {
        // append to dataframe
        parser::parse(&mut store, &mut df);
        // allow main to take control of MutexGuard
        let store_ = store.lock().unwrap();

        // println!("{}", store_.len());
    }

}

