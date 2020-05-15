#[macro_use]
extern crate lazy_static;

use std::error;
use std::fmt;
use std::fs;
use std::process::Command;

use serde::Serialize;

mod keyboard;
mod wireless;
mod storage;
mod processor;
mod thermal;
mod battery;
mod volume;
mod tztime;

const WARNING: &str = "#d79921";
const ALARM: &str = "#fb4934";

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Serialize)]
pub struct Block {
    name: String,
    full_text: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    color: String
}

impl Block {
    fn new(name: String, symb: char, text: String) -> Block {
        Block {
            name: name,
            full_text: format!("{} {}", symb, text),
            color: "".to_string()
        }
    }

    fn new_warning(name: String, symb: char, text: String) -> Block {
        Block {
            name: name,
            full_text: format!("{} {}", symb, text),
            color: WARNING.to_string()
        }
    }

    fn new_alarm(name: String, symb: char, text: String) -> Block {
        Block {
            name: name,
            full_text: format!("{} {}", symb, text),
            color: ALARM.to_string()
        }
    }

}

#[derive(Debug)]
struct BlockError;

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "run error")
    }
}

impl error::Error for BlockError {
    fn description(&self) -> &str{
        "run error"
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

fn run(cmd: &str, args: std::vec::Vec<&str>) -> Result<String> {
    let res = Command::new(cmd).args(&args).output()?;
    if res.status.success() {
        let stdout = String::from_utf8(res.stdout)?;
        return Ok(stdout);
    } else {
        Err(BlockError.into())
    }
}

fn readfile(filename: &str) -> crate::Result<String> {
    match fs::read_to_string(filename) {
        Ok(value) => Ok(value.trim().to_string()),
        Err(_) => Err(BlockError.into())
    }
}

pub fn blocks() -> std::vec::Vec<Block> {
    vec![
        keyboard::get_block(),
        storage::get_block(),
        wireless::get_block(),
        processor::get_block(),
        thermal::get_block(),
        battery::get_block(),
        volume::get_block(),
        tztime::get_block()
    ]
}
