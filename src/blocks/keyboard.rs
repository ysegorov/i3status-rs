
use std::process::Command;

use super::{Block, Status};

pub struct KeyboardBlock;

impl KeyboardBlock {
    fn layout(&self) -> String {
        let output = Command::new("xkblayout-state")
            .args(&["print", "%s"])
            .output()
            .map(|x| {
                String::from_utf8(x.stdout).unwrap_or(String::from("?"))
            })
            .ok();
        match output {
            Some(value) => value,
            None => String::from("!")
        }
    }
}

impl Block for KeyboardBlock {
    fn make(&self) -> (&str, String, Status) {
        let layout = self.layout();
        let status = if layout == "us" { Status::Normal } else { Status::Warning };
        let symb = '';

        let text = format!("{} {}", symb, layout);

        ("keyboard", text, status)
    }
}
