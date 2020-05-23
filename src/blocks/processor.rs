
use sysinfo::{System, SystemExt, ProcessorExt};

use super::{Block, Status};

pub struct ProcessorBlock;

impl ProcessorBlock {
    fn status(&self, load: i64) -> Status {
        match load {
            x if x < 30 => Status::Normal,
            x if x < 70 => Status::Warning,
            _ => Status::Alarm
        }
    }
}

impl Block for ProcessorBlock {
    fn make(&self, s: &mut System) -> (&str, String, Status) {
        s.refresh_cpu();

        let load = s.get_global_processor_info().get_cpu_usage() as i64;
        let status = self.status(load);
        let symb = '';

        let text = format!("{} {}%", symb, load);

        ("processor", text, status)
    }
}
