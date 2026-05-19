use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn write_object(hash: &str, content: &str) -> Result<()> {
    let dir = &hash[..2];
    let file = &hash[2..];

    let object_dir = format!(".gut/objects/{dir}");

    fs::create_dir_all(&object_dir)?;

    let object_path = format!("{object_dir}/{file}");

    fs::write(object_path, content)?;

    Ok(())
}

pub fn read_object(hash: &str) -> Result<String> {
    let dir = &hash[..2];
    let file = &hash[2..];

    let path = format!(".gut/objects/{dir}/{file}");

    Ok(fs::read_to_string(path)?)
}

pub fn gut_exists() -> bool {
    Path::new(".gut").exists()
}