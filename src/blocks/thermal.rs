
use std::fs;

use super::{Block, Status};

const SYSFS_THERMAL_ZONE0_TEMP: &str = "/sys/class/thermal/thermal_zone0/temp";

pub struct ThermalBlock;

impl ThermalBlock {
    fn temp(&self) -> i32 {
        let text = fs::read_to_string(SYSFS_THERMAL_ZONE0_TEMP)
            .map(|x| x.trim().to_owned())
            .ok();
        if let Some(value) = text {
            let value = value.parse::<i32>();
            if value.is_ok() { value.unwrap() / 1000 } else { 0 }
        } else {
            0
        }
    }
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
    fn make(&self) -> (&str, String, Status) {
        let temp = self.temp();
        let symb = self.symb(temp);
        let status = self.status(temp);

        let text = format!("{} {}℃", symb, temp);

        ("thermal", text, status)
    }
}
