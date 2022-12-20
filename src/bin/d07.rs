use std::{collections::HashMap, fs};

fn main() {
    let file = fs::read_to_string("./src/bin/d07.txt").unwrap();
    task1(&file);
    task2(&file);
}

fn parse(input: &str) -> HashMap<String, usize> {
    let mut cur_dir = vec![];
    let mut tree: HashMap<String, usize> = HashMap::new();
    for line in input.split('\n') {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => match parts[2] {
                    ".." => {
                        cur_dir.pop();
                    }
                    _ => {
                        cur_dir.push(parts[2]);
                        let path = cur_dir.join("/");
                        tree.insert(path, 0);
                    }
                },
                "ls" => {
                    continue;
                }
                _ => {
                    panic!("shouldn't happen");
                }
            },
            "dir" => {
                continue;
            }
            _ => {
                let size = parts[0].parse::<usize>().unwrap();

                for (i, _p) in cur_dir.iter().enumerate() {
                    let path_part = cur_dir[0..=i].join("/");
                    let value = tree.get_mut(&path_part);
                    match value {
                        Some(v) => *v += size,
                        None => panic!("no item to update"),
                    }
                }
            }
        }
    }

    tree
}

fn task1(input: &str) {
    let max_size: usize = 100_000;
    let tree = parse(input);
    let mut answer = 0_usize;

    for item in tree.values() {
        if item <= &max_size {
            answer += item;
        }
    }
    dbg!(answer);
}

fn task2(input: &str) {
    let total_disk_space = 70_000_000_usize;
    let space_needed = 30_000_000_usize;
    let tree = parse(input);

    let mut sorted_tree: Vec<&usize> = tree.values().collect();
    sorted_tree.sort();

    let current_used_space = sorted_tree.last().unwrap();
    let current_free_space = total_disk_space - *current_used_space;

    for item in sorted_tree {
        if current_free_space + item >= space_needed {
            dbg!(item);
            break;
        }
    }
}
