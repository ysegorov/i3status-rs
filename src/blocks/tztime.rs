
use chrono::prelude::*;

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
    fn make(&self) -> (&str, String, Status) {
        ("tztime", self.full_text(), Status::Normal)
    }
}

// pub struct TzTimeBlock {
//     name: String,
//     symb: char,
// }

// impl TzTimeBlock {
//     pub fn new() -> TzTimeBlock {
//         TzTimeBlock {
//             name: String::from("tztime"),
//             symb: '',
//         }
//     }
//     fn full_text(&self) -> String {
//         let text = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
//         format!("{} {}", self.symb, text)
//     }
// }

// impl Serialize for TzTimeBlock {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer
//     {
//         let mut map = serializer.serialize_map(Some(2))?;
//         map.serialize_entry("name", &self.name)?;
//         map.serialize_entry("full_text", &self.full_text())?;
//         map.end()
//     }
// }
