
use std::process::Command;

use super::{Block, Status};

pub struct ProcessorBlock;

impl ProcessorBlock {
    fn load(&self) -> i8 {
        let output = Command::new("sh")
            .arg("-c")
            .arg("vmstat -w 5 1 |tail -n 1 |awk '{ print $15; }'")
            .output()
            .map(|x| String::from_utf8(x.stdout).unwrap_or(String::from("0")))
            .map(|x| x.trim().to_owned())
            .ok();
        match output {
            Some(value) => {
                100 - value.parse().unwrap_or(0)
            },
            None => 0
        }
    }
    fn status(&self, load: i8) -> Status {
        match load {
            x if x < 40 => Status::Normal,
            _ => Status::Warning
        }
    }
}

impl Block for ProcessorBlock {
    fn make(&self) -> (&str, String, Status) {
        let load = self.load();
        let status = self.status(load);
        let symb = '';

        let text = format!("{} {}%", symb, load);

        ("processor", text, status)
    }
}
