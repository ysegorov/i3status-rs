
use chrono::prelude::*;
use sysinfo::System;

use super::{Block, Status};

pub struct TzTimeBlock;

impl TzTimeBlock {
    fn full_text(&self) -> String {
        let symb = '';
        let text = Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        format!("{} {}", symb, text)
    }
}

impl Block for TzTimeBlock {
    fn make(&self, _: &mut System) -> (&str, String, Status) {
        ("tztime", self.full_text(), Status::Normal)
    }
}
