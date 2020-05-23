
use std::cell::RefCell;
use sysinfo::System;

use battery::State;

use super::{Block, Status};

pub struct BatteryBlock {
    manager: Option<battery::Manager>,
    battery: Option<RefCell<battery::Battery>>,
}

impl BatteryBlock {
    pub fn new() -> Self {
        let manager = battery::Manager::new().ok();
        let battery = if let Some(mgr) = manager.as_ref() {
            mgr.batteries().map(|mut x| x.next()).unwrap_or(None)
        } else {
            None
        };
        BatteryBlock {
            manager: manager,
            battery: battery.and_then(|x| x.map(|y| RefCell::new(y)).ok()),
        }
    }
    fn symb(&self, capacity: i32) -> char {
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
    fn status(&self, capacity: i32, is_online: bool) -> Status {
        match (capacity, is_online) {
            (x, _) if x < 10 => Status::Alarm,
            (_, y) if !y => Status::Warning,
            _ => Status::Normal,
        }
    }
}

impl Block for BatteryBlock {
    fn make(&self, _: &mut System) -> (&str, String, Status) {
        let name = "battery";

        match (self.manager.as_ref(), self.battery.as_ref()) {
            (Some(manager), Some(battery)) => {
                let mut battery = battery.borrow_mut();
                if manager.refresh(&mut battery).is_ok() {
                    let capacity = (battery.state_of_charge().value * 100.0) as i32;
                    let state = battery.state();
                    let is_online = state != State::Discharging && state != State::Empty;
                    let status = self.status(capacity, is_online);
                    let symb = self.symb(capacity);

                    let text = format!("{} {}%", symb, capacity);

                    (name, text, status)
                } else {
                    (name, String::from("!"), Status::Alarm)
                }
            },
            _ => {
                (name, String::from(""), Status::Alarm)
            }
        }
    }
}
