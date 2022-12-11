use std::fs;

fn main() {
    let file = fs::read_to_string("./src/bin/d02.txt").unwrap();
    task1(&file);
    task2(&file);
}

fn score(o: char, p: char) -> i32 {
    let mut sum = 0;

    let opp = match o {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!("bad input"),
    };

    let player = match p {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("bad input"),
    };

    if opp == player {
        sum += 3 + player
    }
    if opp == player - 1 || opp == player + 2 {
        sum += 6 + player
    }
    if opp == player + 1 || opp == player - 2 {
        sum += player
    }

    sum
}

fn score_2(o: char, p: char) -> i32 {
    let mut sum = 0;

    let opp = match o {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!("bad input"),
    };

    let player = match p {
        'X' => {
            //lose
            if opp == 1 {
                3
            } else {
                opp - 1
            }
        }
        'Y' => {
            //draw
            opp + 3
        }
        'Z' => {
            //win
            if opp == 3 {
                1 + 6
            } else {
                opp + 1 + 6
            }
        }
        _ => panic!("bad input"),
    };

    sum += player;

    sum
}

fn task1(input: &str) {
    let mut overall_score = 0;
    let games: Vec<&str> = input.split('\n').collect();
    for game in &games {
        let (o, p) = match game.split_once(' ') {
            Some((o, p)) => (o.chars().next().unwrap(), p.chars().next().unwrap()),
            None => panic!("something went wrong"),
        };

        overall_score += score(o, p);
    }
    dbg!(overall_score);
}

fn task2(input: &str) {
    let mut overall_score = 0;
    let games: Vec<&str> = input.split('\n').collect();
    for game in &games {
        let (o, p) = match game.split_once(' ') {
            Some((o, p)) => (o.chars().next().unwrap(), p.chars().next().unwrap()),
            None => panic!("something went wrong"),
        };

        overall_score += score_2(o, p);
    }
    dbg!(overall_score);
}
