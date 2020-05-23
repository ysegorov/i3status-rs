
use std::process::Command;
use sysinfo::System;

use super::{Block, Status};

pub struct VolumeBlock;

impl VolumeBlock {
    fn level_state(&self) -> (i8, bool) {
        let output = Command::new("sh")
            .arg("-c")
            .arg("amixer -c 1 -M -D pulse get Master |tail -n 1 |awk '{print $5, $6;}'")
            .output()
            .map(|x| {
                String::from_utf8(x.stdout)
                    .map(|x| x.replace("[", "").replace("]", "").replace("%", ""))
                    .unwrap_or(String::from("0 off"))
            })
            .ok();

        match output {
            Some(value) => {
                let mut parts = value.split_whitespace();
                let level = parts.next();
                let state = parts.next();

                let level: i8 = match level {
                    Some(value) => value.parse().unwrap_or(0),
                    _ => 0
                };
                let is_online = if let Some(value) = state { value == "on" } else { false };

                (level, is_online)
            },
            None => (0, false)
        }
    }
    fn status(&self, level: i8, is_online: bool) -> Status {
        if !is_online {
            Status::Alarm
        } else if level <= 10 {
            Status::Warning
        } else {
            Status::Normal
        }
    }
    fn symb(&self, level: i8) -> char {
        match level {
            x if x <= 10 => '',
            x if x <= 50 => '',
            _ => '',
        }
    }
}

impl Block for VolumeBlock {
    fn make(&self, _: &mut System) -> (&str, String, Status) {
        let (level, is_online) = self.level_state();
        let status = self.status(level, is_online);
        let symb = self.symb(level);

        let text = format!("{} {}%", symb, level);

        ("volume", text, status)
    }
}
