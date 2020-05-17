use std::io::prelude::*;
use std::{io, thread, time};
use std::collections::HashMap;

mod blocks;


fn main() {
    let delay = time::Duration::from_secs(3);
    let blocks = crate::blocks::get_blocks();
    println!("{{\"version\": 1}}");
    println!("[");
    println!("[],");
    loop {
        let data: Vec<HashMap<&str, String>> = blocks.iter().map(|x| x.serialize()).collect();
        println!("{},", serde_json::to_string(&data).unwrap());
        io::stdout().flush().ok();
        thread::sleep(delay);
    }
}
