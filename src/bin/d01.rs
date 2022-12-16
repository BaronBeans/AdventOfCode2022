use std::fs;

fn main() {
    let data = fs::read_to_string(String::from("./src/bin/d01.txt")).expect("file not found");
    task1(&data);
    task2(&data);
}

fn task1(input: &str) {
    // find the elf carrying the most calories:
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut result: Vec<i32> = Vec::new();
    for elf in elves {
        let mut sum = 0;
        for i in elf.split('\n') {
            sum += i.parse::<i32>().unwrap();
        }
        result.push(sum);
    }
    let sum = result.iter().max().copied().unwrap();
    dbg!(sum);
}

fn task2(input: &str) {
    // find the top 3 elves carrying the most calories:
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut result: Vec<i32> = Vec::new();
    let mut answer: Vec<i32> = Vec::new();
    for elf in elves {
        let mut sum = 0;
        for i in elf.split('\n') {
            sum += i.parse::<i32>().unwrap();
        }
        result.push(sum);
    }
    result.sort();

    answer.push(result.pop().unwrap());
    answer.push(result.pop().unwrap());
    answer.push(result.pop().unwrap());

    let mut sum = 0;
    for a in answer {
        sum += a;
    }

    dbg!(sum);
}
