use chrono::prelude::*;

use crate::Block;

pub fn get_block() -> Block {
    let name = "tztime".to_string();
    let symb = '';
    let text = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    Block::new(name, symb, text)
}
