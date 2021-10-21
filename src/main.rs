use std::io::{self, Write};
use std::path::Path;

const LICENSE: &str = r##"Everything in this project is placed into the public domain.
Any and all rights to this software are waived, by any contributors and I."##;

/// Ensures that a directory exists.
fn ensure_directory(dir: &Path) {
    if !dir.exists() {
        std::fs::create_dir(dir).unwrap();
    }
}

fn write_license(folder: &str) {
    let license_path = [folder, "/LICENSE"].concat();
    let mut license_file = std::fs::File::create(license_path).expect("could not create file");
    license_file.write_all(LICENSE.as_bytes()).unwrap();
}

fn add(item: &str, folder: &str) {
    match item {
        "license" => write_license(folder),
        _ => println!("'{}' is an invalid item.", item)
    }
}

fn init(folder: &str) {
    let usable_path = Path::new(folder);
    ensure_directory(usable_path);
    write_license(folder);
    let output = std::process::Command::new("git")
        .args(&["init", folder])
        .output()
        .expect("could not run 'git init'");
    io::stdout().write_all(&output.stdout).unwrap();
    println!("Successfully created project '{}'.", folder);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "add" => add(args[2].as_str(), args[3].as_str()),
        "init" => init(args[2].as_str()),
        _ => println!("Sorry, '{}' is an unsupported operation.", args[1].clone()),
    }
}
