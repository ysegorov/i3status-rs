
use std::collections::HashMap;

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
    fn make(&self) -> (&str, String, Status);

    fn serialize(&self) -> HashMap<&str, String> {
        let (name, text, status) = self.make();
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
}

pub fn get_blocks() -> [Box<dyn Block>; 8] {
    [
        Box::new(keyboard::KeyboardBlock),
        Box::new(storage::StorageBlock),
        Box::new(wireless::WirelessBlock),
        Box::new(processor::ProcessorBlock),
        Box::new(thermal::ThermalBlock),
        Box::new(battery::BatteryBlock::new()),
        Box::new(volume::VolumeBlock),
        Box::new(tztime::TzTimeBlock),
    ]
}
