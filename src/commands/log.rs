use anyhow::Result;
use std::fs;

use crate::object::store;

pub fn run() -> Result<()> {
    let mut current = fs::read_to_string(".gut/refs/heads/main")?
        .trim()
        .to_string();

    if current.is_empty() {
        println!("No commits yet");
        return Ok(());
    }

    loop {
        let object = store::read_object(&current)?;

        println!("commit {}\n", current);
        println!("{}\n", object);

        let parent_line = object
            .lines()
            .find(|l| l.starts_with("parent "));

        match parent_line {
            Some(line) => {
                let hash = line.replace("parent ", "");

                if hash.trim().is_empty() {
                    break;
                }

                current = hash;
            }

            None => break,
        }
    }

    Ok(())
}