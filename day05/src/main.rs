extern crate core;

use std::collections::VecDeque;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::str::Lines;

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
    let mut boxes = Boxes::default();
    boxes.parse_and_run(PART_1_DATA, false);
    println!("{}", boxes.answer());
}

fn part2() {
    let mut boxes = Boxes::default();
    boxes.parse_and_run(PART_1_DATA, true);
    println!("{}", boxes.answer());
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct Boxes([VecDeque<char>; 9]);

impl Boxes {
    pub fn add(&mut self, line: &str) -> bool {
        if line.is_empty() {
            return false;
        }

        let mut rdr = Cursor::new(line);
        let mut group = [0; 3];

        let mut col = 0;
        while rdr.read_exact(&mut group).is_ok() {
            if group[0] == b'[' {
                let label = group[1] as char;
                self.0[col].push_front(label);
            }

            // Skip the next space.  If we fail then we're at the end so exit
            if rdr.seek(SeekFrom::Current(1)).is_err() {
                return true;
            }

            col += 1;
        }

        true
    }

    pub fn add_lines(&mut self, lines: &mut Lines) {
        for line in lines {
            if !self.add(line) {
                return;
            }
        }
    }

    pub fn run(&mut self, instr: Instr, at_once: bool) {
        if at_once {
            let src = &mut self.0[instr.src];
            let idx = src.len() - instr.count;
            let tail = src.split_off(idx);
            self.0[instr.dst].extend(tail);
        } else {
            for _ in 0..instr.count {
                let item = self.0[instr.src].pop_back().unwrap();
                self.0[instr.dst].push_back(item);
            }
        }
    }

    pub fn run_all(&mut self, lines: &mut Lines, at_once: bool) {
        for line in lines {
            if let Ok(instr) = line.try_into() {
                self.run(instr, at_once)
            }
        }
    }

    pub fn parse_and_run(&mut self, problem: &str, at_once: bool) {
        let mut lines = problem.lines();
        self.add_lines(&mut lines);
        self.run_all(&mut lines, at_once);
    }

    pub fn answer(&self) -> String {
        self.0
            .iter()
            .map(|b| b.back().unwrap_or(&' '))
            .collect::<String>()
            .trim()
            .to_string()
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct Instr {
    pub src: usize,
    pub dst: usize,
    pub count: usize,
}

impl Instr {
    pub fn new(src: usize, dst: usize, count: usize) -> Self {
        Instr { src, dst, count }
    }

    pub fn parse(line: &str) -> Option<Self> {
        let mut parts = line.split_whitespace();

        if parts.next().unwrap_or_default() != "move" {
            return None;
        }
        let count = parts.next()?.parse().ok()?;

        if parts.next().unwrap_or_default() != "from" {
            return None;
        }
        let mut src = parts.next()?.parse().ok()?;

        if parts.next().unwrap_or_default() != "to" {
            return None;
        }
        let mut dst = parts.next()?.parse().ok()?;

        if parts.next().is_some() {
            return None;
        }

        src -= 1;
        dst -= 1;

        Some(Instr { count, src, dst })
    }
}

impl TryFrom<&str> for Instr {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        Instr::parse(line).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn make_setup() -> String {
        let parts = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 "
        ];

        parts.join("\n")
    }

    fn make_instr() -> String {
        "
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        "
        .trim()
        .to_string()
    }

    fn make_problem() -> String {
        format!("{}\n\n{}", make_setup(), make_instr())
    }

    #[test]
    fn parse_example_state() {
        let mut boxes = Boxes::default();
        boxes.add_lines(&mut make_problem().lines());

        for (idx, col) in boxes.0.iter().enumerate() {
            match idx {
                0 => assert_eq!(VecDeque::from(['Z', 'N']), *col),
                1 => assert_eq!(VecDeque::from(['M', 'C', 'D']), *col),
                2 => assert_eq!(VecDeque::from(['P']), *col),
                _ => assert!(col.is_empty()),
            }
        }
    }

    #[test]
    fn parse_instr() {
        for (line, want) in make_instr().lines().zip([
            Instr::new(1, 0, 1),
            Instr::new(0, 2, 3),
            Instr::new(1, 0, 2),
            Instr::new(0, 1, 1),
        ]) {
            assert_eq!(Some(want), Instr::parse(line));
        }
    }

    #[test]
    fn test_one_at_a_time() {
        assert_eq!("CMZ", get_ans(false))
    }

    #[test]
    fn test_all_at_once() {
        assert_eq!("MCD", get_ans(true))
    }

    fn get_ans(at_once: bool) -> String {
        let mut boxes = Boxes::default();
        boxes.parse_and_run(&make_problem(), at_once);
        boxes.answer()
    }
}
