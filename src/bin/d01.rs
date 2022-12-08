use std::fs;

fn main() {
    println!("running code from the day");
    let data = fs::read_to_string(String::from("./src/bin/d01.txt")).expect("file not found");
    task1(&data);
    task2(&data);
}
fn task1(input: &str) -> Option<i32> {
    // find the elf carrying the most calories:
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut result: Vec<i32> = Vec::new();
    for elf in elves {
        let mut sum = 0;
        for i in elf.split("\n") {
            sum += i.parse::<i32>().unwrap();
        }
        result.push(sum);
    }
    println!("task1: {:?}", result.iter().max().copied().unwrap());
    result.iter().max().copied()
}
fn task2(input: &str) -> i32 {
    // find the top 3 elves carrying the most calories:
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut result: Vec<i32> = Vec::new();
    for elf in elves {
        let mut sum = 0;
        for i in elf.split("\n") {
            sum += i.parse::<i32>().unwrap();
        }
        result.push(sum);
    }
    result.sort();

    let mut answer: Vec<i32> = vec![];
    answer.push(result.pop().unwrap());
    answer.push(result.pop().unwrap());
    answer.push(result.pop().unwrap());

    let mut sum = 0;
    for a in answer {
        sum += a;
    }

    println!("task 2: {:?}", sum);
    sum
}
