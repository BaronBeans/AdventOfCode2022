use std::{io, process::Command};

fn main() {
    println!("Enter day:");
    let mut day_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut day_input).unwrap_or_default();

    Command::new("cargo")
        .arg("new")
        .arg(format!("../d{}", day_input.trim()))
        .output()
        .expect("something went wrong while creating the template");

    std::process::exit(exitcode::OK);
}
