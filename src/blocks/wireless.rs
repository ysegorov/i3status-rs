
use std::fs;
use std::path::Path;
use std::process::Command;

use super::{Block, Status};

pub struct WirelessBlock;

impl WirelessBlock {
    fn dev_essid(&self) -> (Option<String>, Option<String>) {
        let output = Command::new("sh")
            .arg("-c")
            .arg("iwgetid |sed 's/ESSID:\"\\(.*\\)\"/\\1/'")
            .output()
            .map(|x| String::from_utf8(x.stdout).unwrap_or(String::from("")))
            .ok();
        match output {
            Some(value) => {
                let mut parts = value.split_whitespace()
                    .map(|x| String::from(x));
                let dev = parts.next();
                let essid = parts.next();

                (dev, essid)
            },
            None => (None, None)
        }
    }
    fn operstate(&self, dev: &str) -> String {
        let filename = Path::new("/sys/class/net")
            .join(dev)
            .join("operstate");
        fs::read_to_string(filename.to_str().unwrap())
            .map(|x| String::from(x))
            .map(|x| x.trim().to_owned())
            .unwrap_or(String::from("down"))
    }
    fn ip(&self, dev: &str) -> String {
        let cmd = format!("ip -br -4 addr show dev {} |awk '{{print $3;}}' |awk -F \"/\" '{{print $1;}}'", dev);
        let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .map(|x| String::from_utf8(x.stdout).unwrap_or(String::from("?.?.?.?")))
            .map(|x| x.trim().to_owned())
            .ok();
        match output {
            Some(value) => value,
            None => String::from("?.?.?.?")
        }
    }
    fn quality(&self, dev: &str) -> i8 {
        let cmd = format!("iwconfig {} |grep Quality |awk '{{print $2;}}' |sed 's/Quality=//'", dev);
        let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .map(|x| String::from_utf8(x.stdout).unwrap_or(String::from("")))
            .map(|x| x.trim().to_owned())
            .ok();
        match output {
            Some(value) => {
                let parts: Vec<i8> = value.split('/')
                    .map(|x| x.parse::<i8>().unwrap_or(-1))
                    .collect();
                if parts.len() == 2 {
                    ((parts[0] as f32 / parts[1] as f32) * 100.0) as i8
                } else {
                    0
                }
            },
            None => 0
        }
    }
    fn status(&self, quality: i8) -> Status {
        match quality {
            x if x < 30 => Status::Alarm,
            x if x < 80 => Status::Warning,
            _ => Status::Normal,
        }
    }
}

impl Block for WirelessBlock {
    fn make(&self) -> (&str, String, Status) {
        let (dev, essid) = self.dev_essid();
        let symb = '';

        if dev.is_none() || essid.is_none() {
            let text = format!("{} !", symb);
            return ("wireless", text, Status::Alarm);
        }
        let (dev, essid) = (dev.unwrap(), essid.unwrap());

        let operstate = self.operstate(&dev);
        if operstate != "up" {
            let text = format!("{} !", symb);
            return ("wireless", text, Status::Alarm);
        }
        let ip = self.ip(&dev);
        let quality = self.quality(&dev);
        let status = self.status(quality);

        let text = format!("{} {}% {} {}", symb, quality, essid, ip);

        ("wireless", text, status)
    }
}
