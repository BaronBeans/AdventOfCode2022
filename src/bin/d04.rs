use std::fs;

fn main() {
    let file = fs::read_to_string("./src/bin/d04.txt").unwrap();
    task1(&file);
    task2(&file);
}

fn task1(input: &str) {
    let mut sum = 0;
    for pair in input.split('\n') {
        let pair_tup: Vec<&str> = pair.split(',').collect();
        let (p1, p2): (&str, &str) = (pair_tup[0], pair_tup[1]);
        let p1_tup: Vec<&str> = p1.split('-').collect();
        let (p1_start, p1_end): (&str, &str) = (p1_tup[0], p1_tup[1]);
        let p2_tup: Vec<&str> = p2.split('-').collect();
        let (p2_start, p2_end): (&str, &str) = (p2_tup[0], p2_tup[1]);

        if p2_start.parse::<i32>().unwrap() >= p1_start.parse::<i32>().unwrap()
            && p2_end.parse::<i32>().unwrap() <= p1_end.parse::<i32>().unwrap()
            || p1_start.parse::<i32>().unwrap() >= p2_start.parse::<i32>().unwrap()
                && p1_end.parse::<i32>().unwrap() <= p2_end.parse::<i32>().unwrap()
        {
            sum += 1;
        }
    }
    dbg!(sum);
}

fn task2(input: &str) {
    let mut sum = 0;
    for pair in input.split('\n') {
        let pair_tup: Vec<&str> = pair.split(',').collect();
        let (p1, p2): (&str, &str) = (pair_tup[0], pair_tup[1]);
        let p1_tup: Vec<&str> = p1.split('-').collect();
        let (p1_start, p1_end): (i32, i32) = (
            p1_tup[0].parse::<i32>().unwrap(),
            p1_tup[1].parse::<i32>().unwrap(),
        );
        let p2_tup: Vec<&str> = p2.split('-').collect();
        let (p2_start, p2_end): (i32, i32) = (
            p2_tup[0].parse::<i32>().unwrap(),
            p2_tup[1].parse::<i32>().unwrap(),
        );

        if p2_start >= p1_start && p2_start <= p1_end
            || p2_end >= p1_start && p2_end <= p1_end
            || p1_start >= p2_start && p1_start <= p2_end
            || p1_end >= p2_start && p1_end <= p2_end
        {
            sum += 1;
        }
    }
    dbg!(sum);
}
