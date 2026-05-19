use anyhow::Result;
use std::fs;

pub fn run() -> Result<()> {
    let index = fs::read_to_string(".gut/index")?;

    if index.trim().is_empty() {
        println!("nothing staged");
    } else {
        println!("staged files:\n");
        println!("{}", index);
    }

    Ok(())
}