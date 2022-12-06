extern crate core;

use std::ops::Range;

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
    println!("{}", count_wrapped(PART_1_DATA));
}

fn part2() {
    println!("{}", count_overlapped(PART_1_DATA));
}

fn to_ranges(line: &str) -> (Range<u16>, Range<u16>) {
    let line_bytes = line.as_bytes();

    let mut base = 0;

    let (r1_1, idx) = atoi::FromRadix10::from_radix_10(line_bytes);
    if idx == 0 {
        panic!("Invalid line: {line}")
    } else {
        base += idx + 1;
    }

    let (r1_2, idx) = atoi::FromRadix10::from_radix_10(&line_bytes[base..]);
    if idx == 0 {
        panic!("Invalid line: {line}")
    } else {
        base += idx + 1;
    }

    let (r2_1, idx) = atoi::FromRadix10::from_radix_10(&line_bytes[base..]);
    if idx == 0 {
        panic!("Invalid line: {line}")
    } else {
        base += idx + 1;
    }

    let (r2_2, idx) = atoi::FromRadix10::from_radix_10(&line_bytes[base..]);
    if idx == 0 {
        panic!("Invalid line: {line}")
    }

    (r1_1..r1_2, r2_1..r2_2)
}

fn count_wrapped(data: &str) -> usize {
    let mut count = 0;
    for line in data.lines() {
        let (r1, r2) = to_ranges(line);
        if contains(&r1, &r2) || contains(&r2, &r1) {
            count += 1;
        }
    }
    count
}

#[inline]
fn contains(r1: &Range<u16>, r2: &Range<u16>) -> bool {
    r1.start >= r2.start && r1.end <= r2.end
}

fn count_overlapped(data: &str) -> usize {
    let mut count = 0;
    for line in data.lines() {
        let (r1, r2) = to_ranges(line);
        if overlaps(&r1, &r2) || overlaps(&r2, &r1) {
            count += 1;
        }
    }
    count
}

#[inline]
fn overlaps(r1: &Range<u16>, r2: &Range<u16>) -> bool {
    (r1.start >= r2.start && r1.start <= r2.end) || (r1.end >= r2.start && r1.end <= r2.end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
        .trim()
    }

    #[test]
    fn test_line() {
        assert_eq!((2u16..4, 6u16..8), to_ranges("2-4,6-8"))
    }

    #[test]
    fn example1() {
        assert_eq!(2, count_wrapped(test_data()))
    }

    #[test]
    fn example2() {
        assert_eq!(4, count_overlapped(test_data()))
    }
}
