
use sysinfo::{System, SystemExt, ComponentExt};

use super::{Block, Status};

pub struct ThermalBlock;

impl ThermalBlock {
    fn symb(&self, temp: i32) -> char {
        match temp {
            x if x <= 65 => '',
            x if x <= 75 => '',
            _ => '',
        }
    }
    fn status(&self, temp: i32) -> Status {
        match temp {
            x if x <= 65 => Status::Normal,
            x if x <= 75 => Status::Warning,
            _ => Status::Alarm,
        }
    }
}

impl Block for ThermalBlock {
    fn make(&self, s: &mut System) -> (&str, String, Status) {
        s.refresh_components();
        let temp: Option<i32> = s.get_components().iter()
            .filter(|x| x.get_label().starts_with("Core"))
            .map(|x| x.get_temperature() as i32)
            .max();
        let temp = match temp {
            Some(x) => x,
            None => 0,
        };
        let symb = self.symb(temp);
        let status = self.status(temp);

        let text = format!("{} {}℃", symb, temp);

        ("thermal", text, status)
    }
}
