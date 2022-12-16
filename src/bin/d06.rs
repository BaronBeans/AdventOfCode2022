use std::fs;

fn main() {
    let file = fs::read_to_string("./src/bin/d06.txt").unwrap();
    task1(&file);
    task2(&file);
}

fn task1(input: &str) {
    for (i, w) in input.chars().collect::<Vec<char>>().windows(4).enumerate() {
        let mut seen = vec![];
        let mut dupe = false;
        for c in w {
            if seen.contains(c) {
                // duplicate
                dupe = true;
                break;
            }
            seen.push(*c);
        }
        if !dupe {
            dbg!(i + 4);
            break;
        }
    }
}

fn task2(input: &str) {
    let mut answer = 0;
    for (i, w) in input.chars().collect::<Vec<char>>().windows(14).enumerate() {
        let mut seen = vec![];
        let mut dupe = false;
        for c in w {
            if seen.contains(c) {
                // duplicate
                dupe = true;
                break;
            }
            seen.push(*c);
        }
        if !dupe {
            answer = i as i32 + 14;
            break;
        }
    }
    dbg!(answer);
}
