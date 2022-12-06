extern crate core;

const PART_1: bool = false;

static PART_1_DATA: &str = include_str!("input");

fn main() {
    if PART_1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    println!("{}", play_normal(PART_1_DATA));
}

fn part2() {
    println!("{}", play_optimal(PART_1_DATA));
}

fn comp_score(comp: u8) -> usize {
    (match comp {
        b'a'..=b'z' => comp - b'a' + 1,
        b'A'..=b'Z' => comp - b'A' + 27,
        _ => panic!("Unknown char: {}", comp as char),
    }) as usize
}

fn play_normal(data: &str) -> u64 {
    let mut score = 0;

    for line in data.lines() {
        let mut found = [false; 53];
        let (p1, p2) = line.split_at(line.len() / 2);

        for c in p1.chars() {
            found[comp_score(c as u8)] = true
        }

        for c in p2.chars() {
            let cs = comp_score(c as u8);
            if found[cs] {
                score += cs as u64;
                break;
            }
        }
    }

    score
}

fn play_optimal(data: &str) -> u64 {
    let mut score = 0;

    let mut iter = data.lines();

    while let Some(g1) = iter.next() {
        let mut found1 = [false; 53];
        for c in g1.chars() {
            found1[comp_score(c as u8)] = true
        }

        let mut found2 = [false; 53];
        for c in iter.next().unwrap().chars() {
            found2[comp_score(c as u8)] = true
        }

        for c in iter.next().unwrap().chars() {
            let cs = comp_score(c as u8);
            if found1[cs] && found2[cs] {
                score += cs as u64;
                break;
            }
        }
    }

    score
}
