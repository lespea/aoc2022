extern crate core;

use std::fmt::{Display, Formatter};

const PART_1: bool = false;

static PART_1_DATA: &str = include_str!("input");

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Choice {
    Rock,
    Scissors,
    Paper,
}

impl Display for Choice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Choice::Rock => "Rock",
            Choice::Paper => "Paper",
            Choice::Scissors => "Scissors",
        })
    }
}

impl Choice {
    fn parse_line(line: &str, proper_decode: bool) -> (Choice, Choice) {
        let mut chars = line.chars();

        let theirs = Choice::from_char(chars.next().unwrap());
        chars.next().unwrap();

        let my_char = chars.next().unwrap();

        let mine = if proper_decode {
            theirs.pick_choice(my_char)
        } else {
            Choice::from_char(my_char)
        };

        (theirs, mine)
    }

    fn from_char(c: char) -> Choice {
        match c {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => panic!("Unknown choice: {c}"),
        }
    }

    fn pick_choice(&self, c: char) -> Choice {
        if c == 'Y' {
            return *self;
        }

        match self {
            Choice::Rock => match c {
                'X' => Choice::Scissors,
                'Z' => Choice::Paper,
                _ => panic!("Unknown choice: {c}"),
            },

            Choice::Paper => match c {
                'X' => Choice::Rock,
                'Z' => Choice::Scissors,
                _ => panic!("Unknown choice: {c}"),
            },

            Choice::Scissors => match c {
                'X' => Choice::Paper,
                'Z' => Choice::Rock,
                _ => panic!("Unknown choice: {c}"),
            },
        }
    }

    fn score(&self) -> u64 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn round_score(&self, that: &Choice) -> u64 {
        const WIN: u64 = 6;
        const TIE: u64 = 3;
        const LOSE: u64 = 0;

        match (self, that) {
            (Choice::Rock, Choice::Rock) => TIE,
            (Choice::Paper, Choice::Paper) => TIE,
            (Choice::Scissors, Choice::Scissors) => TIE,

            (Choice::Rock, Choice::Paper) => LOSE,
            (Choice::Rock, Choice::Scissors) => WIN,

            (Choice::Paper, Choice::Rock) => WIN,
            (Choice::Paper, Choice::Scissors) => LOSE,

            (Choice::Scissors, Choice::Paper) => WIN,
            (Choice::Scissors, Choice::Rock) => LOSE,
        }
    }
}

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

fn play_normal(data: &str) -> u64 {
    let mut score = 0;

    for line in data.lines() {
        let (theirs, mine) = Choice::parse_line(line, false);
        score += mine.score() + mine.round_score(&theirs);
    }

    score
}

fn play_optimal(data: &str) -> u64 {
    let mut score = 0;

    for line in data.lines() {
        let (theirs, mine) = Choice::parse_line(line, true);
        // println!("{theirs} - {mine}");
        score += mine.score() + mine.round_score(&theirs);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &str = "A Y\nB X\nC Z";

    #[test]
    fn normal() {
        let score = play_normal(DATA);
        assert_eq!(15, score);
    }

    #[test]
    fn optimal() {
        let score = play_optimal(DATA);
        assert_eq!(12, score);
    }
}
