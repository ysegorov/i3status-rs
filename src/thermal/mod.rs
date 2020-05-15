
const SYSFS_THERMAL_ZONE0: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn get_block() -> crate::Block {
    let name = "temp".to_string();
    let temp = crate::readfile(SYSFS_THERMAL_ZONE0).unwrap_or("0".to_string()).parse::<i32>();
    let temp: i32 = if temp.is_ok() { temp.unwrap() / 1000 } else { 0 };

    let text = format!("{}℃", temp);

    match temp {
        x if x <= 65 => {
            let symb = '';
            crate::Block::new(name, symb, text)
        },
        x if x <= 75 => {
            let symb = '';
            crate::Block::new_warning(name, symb, text)
        },
        _ => {
            let symb = '';
            crate::Block::new_alarm(name, symb, text)
        }
    }
}
