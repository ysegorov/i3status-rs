
const SYSFS_BAT0: &str = "/sys/class/power_supply/BAT0/capacity";
const SYSFS_AC0: &str = "/sys/class/power_supply/AC0/online";

pub fn get_block() -> crate::Block {
    let name = "battery".to_string();
    let capacity = crate::readfile(SYSFS_BAT0).unwrap_or("0".to_string()).parse();
    let capacity: i8 = if capacity.is_err() { 0 } else { capacity.unwrap() };
    let is_online = crate::readfile(SYSFS_AC0).unwrap_or("1".to_string());
    let is_online = if is_online == "1" { true } else { false };

    let text = format!("{}%", capacity);

    let symb = match capacity {
        0 => '',
        1..=10 => '',
        11..=20 => '',
        21..=30 => '',
        31..=40 => '',
        41..=50 => '',
        51..=60 => '',
        61..=70 => '',
        71..=80 => '',
        81..=90 => '',
        _ => ''
    };

    match (capacity, is_online) {
        (x, _) if x < 10 => crate::Block::new_alarm(name, symb, text),
        (_, y) if !y => crate::Block::new_warning(name, symb, text),
        _ => crate::Block::new(name, symb, text)
    }
}
