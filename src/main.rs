use std::io::prelude::*;
use std::{io, thread, time};


fn main() {
    let delay = time::Duration::from_millis(3 * 1000);
    println!("{{\"version\": 1}}");
    println!("[");
    println!("[],");
    loop {
        println!("{},", serde_json::to_string(&i3status::blocks()).unwrap());
        io::stdout().flush().ok();
        thread::sleep(delay);
    }
}
