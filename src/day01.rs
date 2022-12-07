fn main() {
    let data = aoc::read_as_string(String::from("./input.txt"));
    println!("part1: {:?}", solve1(&data).unwrap());
    println!("part2: {:?}", solve2(&data));
}

fn solve1(data: &str) -> Option<i32> {
    // find the elf carrying the most calories:
    let elves: Vec<&str> = data.split("\n\n").collect();
    let mut result: Vec<i32> = Vec::new();
    for elf in elves {
        let mut sum = 0;
        for i in elf.split("\n") {
            sum += i.parse::<i32>().unwrap();
        }
        result.push(sum);
    }
    result.iter().max().copied()
}

fn solve2(data: &str) -> i32 {
    // find the top 3 elves carrying the most calories:
    let elves: Vec<&str> = data.split("\n\n").collect();
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

    sum
}
