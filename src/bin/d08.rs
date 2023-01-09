#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;

fn main() {
    let file = fs::read_to_string("./src/bin/d08.txt").unwrap();
    task1(&file);
    task2(&file);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = vec![];
    let bounds = input.split('\n').count();
    for _i in 1..=bounds {
        output.push(vec![]);
    }

    for (i, line) in input.split('\n').enumerate() {
        for ch in line.chars() {
            output[i].push(ch);
        }
    }

    output
}

fn task1(input: &str) {
    let mut visible_count = 0;
    let grid = parse(input);
    let bounds = grid.len();

    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if i == 0 || i == bounds - 1 || j == 0 || j == bounds - 1 {
                continue;
            }

            let mut up: Vec<char> = vec![];
            let mut down: Vec<char> = vec![];
            let mut left: Vec<char> = vec![];
            let mut right: Vec<char> = vec![];

            for x in 0..bounds {
                for y in 0..bounds {
                    let ch_new = grid[y].get(x).unwrap();

                    if x == j && y < i {
                        up.push(*ch_new);
                    }
                    if x == j && y > i {
                        down.push(*ch_new);
                    }
                    if x < j && y == i {
                        left.push(*ch_new);
                    }
                    if x > j && y == i {
                        right.push(*ch_new);
                    }
                }
            }

            let num = ch.to_digit(10);

            if up.iter().all(|u| u.to_digit(10) < num) {
                visible_count += 1;
                continue;
            }
            if down.iter().all(|d| d.to_digit(10) < num) {
                visible_count += 1;
                continue;
            }
            if left.iter().all(|l| l.to_digit(10) < num) {
                visible_count += 1;
                continue;
            }
            if right.iter().all(|r| r.to_digit(10) < num) {
                visible_count += 1;
                continue;
            }
        }
    }

    let answer = visible_count + bounds * 4 - 4;
    dbg!(answer);
}

fn task2(input: &str) {
    let mut max_score = 0;
    let grid = parse(input);
    let width = grid.first().unwrap().len();
    let height = grid.len();

    for (y, line) in grid[1..height].iter().enumerate() {
        for (x, ch) in line[1..width].iter().enumerate() {
            let num = ch.to_digit(10).unwrap();

            let mut up = vec![];
            let mut down = vec![];
            let mut left = vec![];
            let mut right = vec![];

            for (_, u) in grid[..=y].iter().rev().enumerate() {
                let i = u.get(x + 1).unwrap();
                up.push(i);
                if i.to_digit(10).unwrap() >= num {
                    break;
                }
            }

            for (_, d) in grid[(y + 2)..].iter().enumerate() {
                let i = d.get(x + 1).unwrap();
                down.push(i);
                if i.to_digit(10).unwrap() >= num {
                    break;
                }
            }

            for (_, l) in grid[y + 1][..=x].iter().rev().enumerate() {
                left.push(l);
                if l.to_digit(10).unwrap() >= num {
                    break;
                }
            }

            for (_, r) in grid[y + 1][(x + 2)..].iter().enumerate() {
                right.push(r);
                if r.to_digit(10).unwrap() >= num {
                    break;
                }
            }

            let score = up.len() * down.len() * left.len() * right.len();

            if score > max_score {
                max_score = score;
            }
        }
    }
    dbg!(max_score);
}
