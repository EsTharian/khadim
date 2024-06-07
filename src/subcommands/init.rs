use colored::Colorize;
use std::fs;

pub fn main(name: Option<String>, path: Option<String>) {
    println!(
        "{} {} on {}",
        "Creating a new Clove.rs project".to_string(),
        name.as_ref().unwrap().green().bold(),
        fs::canonicalize(&path.as_ref().unwrap())
            .unwrap()
            .to_str()
            .unwrap()
            .green()
            .bold()
    );
}
