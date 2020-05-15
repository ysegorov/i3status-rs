
fn extract_data(output: crate::Result<String>) -> crate::Result<(f32, i8)> {
    match output {
        Err(value) => Err(value),
        Ok(value) => {
            let mut data = value.lines().last().unwrap_or("").split_whitespace();
            let value = data.nth(3);
            let percent = data.nth(0);

            match (value, percent) {
                (None, _) | (_, None) => Err(crate::BlockError.into()),
                (Some(val), Some(pct)) => {
                    let val: f32 = val.parse::<f32>().unwrap() / 1024.0 / 1024.0;
                    let pct: i8 = pct.trim_end_matches('%').parse().unwrap();

                    Ok((val, pct))
                }
            }
        }
    }
}

pub fn get_block() -> crate::Block {
    let name = "storage".to_string();
    let symb = '';
    let output = crate::run("df", vec!["-P", "-l", "/"]);

    let data = extract_data(output);

    match data {
        Err(_) => crate::Block::new_alarm(name, symb, "!".to_string()),
        Ok((value, percent)) => {
            let text = format!("{:.1}Gb", value);

            match percent {
                x if x < 95 => crate::Block::new(name, symb, text),
                x if x < 98 => crate::Block::new_warning(name, symb, text),
                _ => crate::Block::new_alarm(name, symb, text)
            }
        }
    }
}
