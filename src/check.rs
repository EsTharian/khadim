use std::{process::Command, vec};

pub fn requirements() {
    let required_commands: Vec<&str> = vec!["cargo", "rustc", "git"];
    let mut missing_commands: Vec<&str> = Vec::new();

    for command in required_commands {
        match Command::new("which").arg(command).output() {
            Ok(result) => {
                if !result.status.success() {
                    missing_commands.push(command);
                }
            }
            Err(result) => println!("{}", result),
        };
    }

    if !missing_commands.is_empty() {
        eprintln!(
            "These commands are required: {:?}",
            missing_commands.join(", ")
        );
    }
}
