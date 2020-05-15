
fn extract_data(output: crate::Result<String>) -> crate::Result<i8> {
    match output {
        Err(value) => Err(value),
        Ok(value) => {
            let data = value.lines().last().unwrap_or("");
            let data = data.split_whitespace().nth(14);

            match data {
                None => Err(crate::BlockError.into()),
                Some(load) => {
                    let load: i8 = load.parse().unwrap();
                    Ok(100 - load)
                }
            }
        }
    }
}

pub fn get_block() -> crate::Block {
    let name = "processor".to_string();
    let symb = '';
    let output = crate::run("vmstat", vec!["-w"]);

    let data = extract_data(output);

    match data {
        Err(_) => crate::Block::new_alarm(name, symb, "!".to_string()),
        Ok(value) => {
            let text = format!("{}%", value);

            match value {
                x if x < 40 => crate::Block::new(name, symb, text),
                _ => crate::Block::new_warning(name, symb, text)
            }
        }
    }
}
