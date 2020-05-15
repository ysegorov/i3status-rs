
use std::path::Path;

fn extract_dev_essid(stat: String) -> (Option<String>, Option<String>) {
    let parts: Vec<String> = stat.split_whitespace().map(|x| x.to_string()).collect();
    let dev = parts.get(0).cloned();
    let essid = match parts.get(1) {
        None => None,
        Some(value) => {
            let value = value.split(':').nth(1);
            if let Some(name) = value {
                Some(name.trim_matches('"').to_string())
            } else {
                None
            }
        }
    };

    (dev, essid)
}

fn extract_ip(dev: &str) -> String {
    let info = crate::run("ip", vec!["-br", "-4", "addr", "show", "dev", dev]).unwrap_or("".to_string());
    let parts: Vec<&str> = info.split_whitespace().collect();
    let ip = parts.get(2).cloned();

    match ip {
        Some(value) => value.to_string(),
        None => "?.?.?.?".to_string()
    }
}

fn extract_quality(dev: &str) -> i8 {
    let info = crate::run("iwconfig", vec![dev]).unwrap_or("".to_string());
    let lines: Vec<&str> = info.lines()
        .filter(|x| x.contains("Link Quality"))
        .map(|mut x| {
            x = x.trim();
            x = if x.starts_with("Link Quality=") { &x[13..] } else { x };
            let pos = x.find(' ').unwrap_or(1);
            &x[..pos]
        })
        .collect();
    let line = lines.get(0).cloned();

    match line {
        None => 0,
        Some(text) => {
            let parts: Vec<i8> = text.split('/')
                .map(|x| x.parse::<i8>().unwrap_or(-1))
                .collect();
            if parts.len() == 2 {
                let left = parts[0] as f32;
                let right = parts[1] as f32;
                ((left / right) * 100.0) as i8
            } else {
                0
            }
        }
    }
}

pub fn get_block() -> crate::Block {
    let name = "wireless".to_string();
    let symb = '';
    let stat = crate::run("iwgetid", vec![]);

    if let Err(_) = stat {
        return crate::Block::new_alarm(name, symb, "!".to_string());
    }

    let (dev, essid) = extract_dev_essid(stat.unwrap());
    if dev.is_none() || essid.is_none() {
        return crate::Block::new_alarm(name, symb, "!".to_string());
    }
    let (dev, essid) = (dev.unwrap(), essid.unwrap());

    let operstate_filename = Path::new("/sys/class/net").join(&dev).join("operstate");
    let operstate = crate::readfile(operstate_filename.to_str().unwrap()).unwrap_or("down".to_string());

    if operstate != "up" {
        return crate::Block::new_warning(name, symb, "0%".to_string());
    }
    let ip = extract_ip(&dev);
    let quality = extract_quality(&dev);

    let text = format!("{}% {} {}", quality, essid, ip);

    crate::Block::new(name, symb, text.to_string())
}
