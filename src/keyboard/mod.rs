
pub fn get_block() -> crate::Block {
    let name = "keyboard".to_string();
    let symb = '';
    let output = crate::run("xkblayout-state", vec!["print", "%s"]);

    match output {
        Err(_value) => crate::Block::new_alarm(name, symb, "!".to_string()),
        Ok(value) => {
            let text = value.trim();
            match text {
                "us" => crate::Block::new(name, symb, text.to_string()),
                _ => crate::Block::new_warning(name, symb, text.to_string())
            }
        }
    }
}
