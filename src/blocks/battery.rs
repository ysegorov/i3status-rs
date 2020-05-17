
use std::fs;

use super::{Block, Status};

const SYSFS_BAT0_CAPACITY: &str = "/sys/class/power_supply/BAT0/capacity";
const SYSFS_AC0_ONLINE: &str = "/sys/class/power_supply/AC0/online";

pub struct BatteryBlock;

impl BatteryBlock {
    fn capacity(&self) -> i8 {
        let text = fs::read_to_string(SYSFS_BAT0_CAPACITY)
            .map(|x| x.trim().to_owned())
            .ok();
        if let Some(value) = text {
            let value = value.parse();
            if value.is_ok() { value.unwrap() } else { 0 }
        } else {
            0
        }
    }
    fn is_online(&self) -> bool {
        let text = fs::read_to_string(SYSFS_AC0_ONLINE)
            .map(|x| x.trim().to_owned())
            .ok();
        if let Some(value) = text {
            value == "1"
        } else {
            false
        }
    }
    fn symb(&self, capacity: i8) -> char {
        match capacity {
            1..=10 => '',
            11..=20 => '',
            21..=30 => '',
            31..=40 => '',
            41..=50 => '',
            51..=60 => '',
            61..=70 => '',
            71..=80 => '',
            81..=90 => '',
            91..=100 => '',
            _ => '',
        }
    }
    fn status(&self, capacity: i8, is_online: bool) -> Status {
        match (capacity, is_online) {
            (x, _) if x < 10 => Status::Alarm,
            (_, y) if !y => Status::Warning,
            _ => Status::Normal,
        }
    }
}

impl Block for BatteryBlock {
    fn make(&self) -> (&str, String, Status) {
        let capacity = self.capacity();
        let is_online = self.is_online();
        let status = self.status(capacity, is_online);
        let symb = self.symb(capacity);

        let text = format!("{} {}%", symb, capacity);

        ("battery", text, status)
    }
}
