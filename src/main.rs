use std::io::prelude::*;
use std::{io, thread, time};
use std::collections::HashMap;

mod blocks;


fn main() {
    let delay = time::Duration::from_secs(2);
    let blocks = blocks::Blocks::new();

    println!("{{\"version\": 1}}");
    println!("[");
    println!("[],");

    loop {
        let data: Vec<HashMap<&str, String>> = blocks.serialize();

        println!("{},", serde_json::to_string(&data).unwrap());
        io::stdout().flush().ok();
        thread::sleep(delay);
    }
}
