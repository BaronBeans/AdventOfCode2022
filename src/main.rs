use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
    process::Command,
    str::FromStr,
};

const CARGO_ROOT: &str = env!("CARGO_MANIFEST_DIR");

const FILE_CONTENTS: &str = r#"
fn main() {
    println!("running code from the day");
    // task1();
    // task2();
}
// fn task1(input: &str) {
//     todo!()
// }
// fn task2(input: &str) {
//     todo!()
// }
"#;

fn main() {
    println!("Enter day:");
    let mut day_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut day_input).unwrap_or_default();

    let base_file_name = format!("d{}", day_input.trim());
    let rust_file_name = format!("{}.rs", base_file_name);
    let input_file_name = format!("{}.txt", base_file_name);
    let mut rust_path = PathBuf::from_str(CARGO_ROOT).unwrap();
    rust_path.push("src");
    rust_path.push("bin");
    rust_path.push(&rust_file_name);
    let mut input_path = PathBuf::from_str(CARGO_ROOT).unwrap();
    input_path.push("src");
    input_path.push("bin");
    input_path.push(&input_file_name);

    if rust_path.exists() {
        // println!("file exists - running binary");
        Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg("--bin")
            .arg(&base_file_name)
            .arg("-q")
            .spawn()
            .expect("something went wrong running the binary")
            .wait()
            .expect("");
    } else {
        println!("creating file");
        let mut file = File::create(rust_path).unwrap();
        file.write_all(FILE_CONTENTS.as_bytes()).unwrap();
        File::create(input_path).unwrap();
    }

    std::process::exit(exitcode::OK);
}
