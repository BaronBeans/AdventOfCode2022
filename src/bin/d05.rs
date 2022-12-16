use std::fs;

struct Instruction {
    mov: usize,
    from: usize,
    to: usize,
}

fn main() {
    let file = fs::read_to_string("./src/bin/d05.txt").unwrap();
    task1(&file);
    task2(&file);
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut crates: Vec<Vec<char>> = vec![];

    for _ in 1..=9 {
        crates.push(vec![]);
    }

    for line in stacks.split('\n') {
        if line.starts_with(" 1") {
            break;
        }
        for (j, chunk) in line.as_bytes().chunks(4).enumerate() {
            let int_char = chunk[1] as char;
            if int_char != ' ' {
                crates[j].push(int_char);
            }
        }
    }

    let mut crates_reversed: Vec<Vec<char>> = vec![];
    for cr in crates {
        crates_reversed.push(cr.into_iter().rev().collect());
    }

    let mut new_instructions: Vec<_> = vec![];

    for i in instructions.split('\n') {
        let splt: Vec<&str> = i.split(' ').collect();
        let mov = splt[1].parse::<usize>().unwrap();
        let from = splt[3].parse::<usize>().unwrap();
        let to = splt[5].parse::<usize>().unwrap();

        new_instructions.push(Instruction { mov, from, to });
    }

    (crates_reversed, new_instructions)
}

fn task1(input: &str) {
    let (crates, _instructions) = parse_input(input);
    let mut new_crates = crates;

    for i in _instructions {
        for _ in 1..=i.mov {
            let item = new_crates[i.from - 1].pop().unwrap();
            new_crates[i.to - 1].push(item);
        }
    }

    let mut output = String::new();
    for mut stack in new_crates {
        let last = stack.pop().unwrap();
        output.push(last);
    }

    dbg!(output);
}

fn task2(input: &str) {
    let (crates, _instructions) = parse_input(input);
    let mut new_crates = crates;

    for i in _instructions {
        let mut items = vec![];
        for _ in 1..=i.mov {
            items.push(new_crates[i.from - 1].pop().unwrap());
        }
        let reversed_items: Vec<&char> = items.iter().rev().collect();
        for x in reversed_items {
            new_crates[i.to - 1].push(*x);
        }
    }

    let mut output = String::new();
    for mut stack in new_crates {
        let last = stack.pop().unwrap();
        output.push(last);
    }

    dbg!(output);
}
