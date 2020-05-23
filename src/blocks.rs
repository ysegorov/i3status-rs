
use std::cell::RefCell;
use std::collections::HashMap;

use sysinfo::{System, SystemExt};

mod keyboard;
mod storage;
mod wireless;
mod processor;
mod thermal;
mod battery;
mod volume;
mod tztime;

const WARNING: &str = "#d79921";
const ALARM: &str = "#fb4934";

pub enum Status {
    Normal,
    Warning,
    Alarm,
}

pub trait Block {
    fn make(&self, s: &mut System) -> (&str, String, Status);
}

fn serialize(info: (&str, String, Status)) -> HashMap<&str, String> {
    let (name, text, status) = info;
    let mut map = HashMap::new();

    map.insert("name", String::from(name));
    map.insert("full_text", text);

    match status {
        Status::Warning => map.insert("color", String::from(WARNING)),
        Status::Alarm => map.insert("color", String::from(ALARM)),
        Status::Normal => None
    };

    map
}

pub struct Blocks {
    sysinfo: RefCell<System>,
    blocks: [Box<dyn Block>; 8],
}

impl Blocks {
    pub fn new() -> Self {
        let mut s = System::new();
        s.refresh_cpu();
        s.refresh_components_list();
        s.refresh_disks_list();

        Blocks {
            sysinfo: RefCell::new(s),
            blocks: [
                Box::new(keyboard::KeyboardBlock),
                Box::new(storage::StorageBlock),
                Box::new(wireless::WirelessBlock),
                Box::new(processor::ProcessorBlock),
                Box::new(thermal::ThermalBlock),
                Box::new(battery::BatteryBlock::new()),
                Box::new(volume::VolumeBlock),
                Box::new(tztime::TzTimeBlock),
            ],
        }
    }
    pub fn serialize(&self) -> Vec<HashMap<&str, String>> {
        let mut s = self.sysinfo.borrow_mut();
        self.blocks.iter().map(|x| x.make(&mut s)).map(serialize).collect()
    }
}
