
use regex::Regex;

lazy_static! {
    static ref VOL_RE: Regex = Regex::new(r"\[(?P<percent>\d{1,3})%\]\s*\[(?P<status>on|off)\]").unwrap();
}

fn extract_data(output: crate::Result<String>) -> crate::Result<(i8, String)> {
    match output {
        Ok(value) => {
            let data = value.lines().last().unwrap_or("");
            match VOL_RE.captures(data) {
                None => Err(crate::BlockError.into()),
                Some(caps) => {
                    let percent = caps.name("percent").map_or("0", |m| m.as_str());
                    let percent: i8 = String::from(percent).parse().unwrap();
                    let status = caps.name("status").map_or("off", |m| m.as_str());

                    Ok((percent, status.to_string()))
                }
            }
        },
        Err(value) => Err(value)
    }
}

pub fn get_block() -> crate::Block {
    let name = "volume".to_string();
    let output = crate::run("amixer", vec!["-c", "1", "-M", "-D", "pulse", "get", "Master"]);

    let data = extract_data(output);

    match data {
        Err(_) => crate::Block::new_alarm(name, '', "!".to_string()),
        Ok((percent, status)) => {

            let text = format!("{}%", percent);

            match (percent, status) {
                (x, y) if x <= 10 || y == "off" => {
                    let symb = '';
                    crate::Block::new_warning(name, symb, text)
                },
                (x, _) => {
                    let symb = if x <= 50 { '' } else { '' };
                    crate::Block::new(name, symb, text)
                }
            }
        }
    }

}
