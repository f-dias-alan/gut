use anyhow::{bail, Result};
use chrono::Local;
use std::fs;
use std::path::Path;

use crate::object::store;
use crate::utils::sha1_hex;

pub fn run(message: String) -> Result<()> {
    if !Path::new(".gut").exists() {
        bail!("not a gut repository");
    }

    let index = fs::read_to_string(".gut/index")
        .unwrap_or_default();

    if index.trim().is_empty() {
        bail!("nothing to commit");
    }

    let parent = fs::read_to_string(".gut/refs/heads/main")
        .unwrap_or_default();

    let author = std::env::var("GUT_AUTHOR")
        .unwrap_or("Unknown".to_string());

    let date = Local::now().to_rfc3339();

    let commit_content = format!(
        "commit\nparent {}\nauthor {}\ndate {}\n\n{}\n\n{}",
        parent.trim(),
        author,
        date,
        message,
        index
    );

    let hash = sha1_hex(commit_content.as_bytes());

    store::write_object(&hash, &commit_content)?;

    fs::write(".gut/refs/heads/main", &hash)?;

    fs::write(".gut/index", "")?;

    println!("Committed as {}", hash);

    Ok(())
}