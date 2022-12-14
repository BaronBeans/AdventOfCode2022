use std::fs;

fn main() {
    let file = fs::read_to_string("./src/bin/d03.txt").unwrap();
    task1(&file);
    task2(&file);
}

fn task1(input: &str) {
    let mut sum = 0;
    for line in input.split('\n') {
        let mut seen_match = false;
        let (part1, part2) = line.split_at(line.len() / 2);
        for x in part1.chars() {
            if seen_match {
                break;
            }
            for y in part2.chars() {
                if y == x {
                    let priority = match y {
                        'a'..='z' => y as u8 - 'a' as u8 + 1,
                        'A'..='Z' => y as u8 - 'A' as u8 + 27,
                        _ => 0,
                    } as i32;
                    sum += priority;
                    seen_match = true;
                    break;
                }
            }
        }
    }
    dbg!(sum);
}

fn task2(input: &str) {
    let mut sum = 0;

    let backpacks: Vec<&str> = input.split('\n').collect();
    for group in backpacks.chunks(3) {
        for ch in group[0].chars() {
            if group[1].contains(ch) && group[2].contains(ch) {
                let priority = match ch {
                    'a'..='z' => ch as u8 - 'a' as u8 + 1,
                    'A'..='Z' => ch as u8 - 'A' as u8 + 27,
                    _ => 0,
                } as i32;
                sum += priority;
                break;
            }
        }
    }
    dbg!(sum);
}
