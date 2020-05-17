
use std::process::Command;

use super::{Block, Status};

pub struct StorageBlock;

impl StorageBlock {
    fn available_capacity(&self) -> (f32, i8) {
        // NB. outputs Available and Capacity fields from df output
        let output = Command::new("sh")
            .arg("-c")
            .arg("df -P -l / |grep -v Avail |awk '{ gsub(\"%\", \"\"); print $4, $5; }'")
            .output()
            .map(|x| {
                String::from_utf8(x.stdout).unwrap_or(String::from("0 0"))
            })
            .ok();
        match output {
            Some(value) => {
                let mut parts = value.split_whitespace();
                let available = parts.next();
                let capacity = parts.next();

                match (available, capacity) {
                    (Some(avail), Some(cap)) => {
                        let avail: f32 = avail.parse().unwrap_or(0.0) /1024.0 / 1024.0;
                        let cap: i8 = cap.parse().unwrap_or(-1);
                        (avail, cap)
                    },
                    _ => (0.0, -1)
                }
            },
            None => (0.0, -1)
        }
    }
    fn status(&self, cap: i8) -> Status {
        match cap {
            0..=95 => Status::Normal,
            96..=98 => Status::Warning,
            _ => Status::Alarm,
        }

    }
}

impl Block for StorageBlock {
    fn make(&self) -> (&str, String, Status) {
        let (avail, cap) = self.available_capacity();
        let status = self.status(cap);
        let symb = '';
        let text = format!("{} {:.1}Gb", symb, avail);

        ("storage", text, status)
    }
}
