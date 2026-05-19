use std::fs;

pub fn run() {
    fs::create_dir(".gut").unwrap();
    fs::create_dir_all(".gut/objects").unwrap();
    fs::create_dir_all(".gut/refs/heads").unwrap();

    fs::write(".gut/HEAD", "ref: refs/heads/main\n").unwrap();
    fs::write(".gut/refs/heads/main", "").unwrap();
    fs::write(".gut/index", "").unwrap();

    println!("Initialized empty gut repository");
}