use std::{
    fs::File,
    io::{self, Read},
    process::Command,
};

fn main() {
    println!("Enter day:");
    let mut day_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut day_input).unwrap_or_default();

    let path = format!("../d{}", day_input.trim());
    Command::new("cargo")
        .arg("new")
        .arg(&path)
        .output()
        .expect("something went wrong while creating the template");

    File::create(format!("{}/input.txt", &path.trim())).unwrap();

    let mut file = File::open(format!("{}/src/main.rs", &path.trim())).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", &contents);

    std::process::exit(exitcode::OK);
}
