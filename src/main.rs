use std::sync::{Arc, Mutex};

mod socks;

fn main() {
    let mut store: Arc<Mutex<Vec<Vec<String>>>> = Arc::new(Mutex::new(Vec::new()));

    socks::print_hello();
    socks::receive_data(&mut store);
    
    loop {
        println!("{:?}", store.lock().len())
    }

}

