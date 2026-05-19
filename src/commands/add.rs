use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

use crate::object::store;
use crate::utils::sha1_hex;

pub fn run(files: Vec<String>) -> Result<()> {
    if !Path::new(".gut").exists() {
        bail!("not a gut repository");
    }

    let mut index = fs::read_to_string(".gut/index")
        .unwrap_or_default();

    for file in files {
        if !Path::new(&file).exists() {
            bail!("file '{}' does not exist", file);
        }

        let content = fs::read(&file)?;

        let hash = sha1_hex(&content);

        let object_content = format!(
            "blob\n{}",
            String::from_utf8_lossy(&content)
        );

        store::write_object(&hash, &object_content)?;

        index.push_str(&format!("{} {}\n", hash, file));

        println!("added {}", file);
    }

    fs::write(".gut/index", index)?;

    Ok(())
}