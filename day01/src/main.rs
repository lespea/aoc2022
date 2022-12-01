use atoi::atoi;
use bstr::ByteSlice;

const PART_1: bool = false;

static PART_1_DATA: &[u8] = include_bytes!("input1");

fn main() {
    if PART_1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    println!("{}", max(PART_1_DATA));
}

fn part2() {
    println!("{}", top_3(PART_1_DATA));
}

fn max(data: &[u8]) -> u64 {
    *get_cals(data).iter().max().unwrap()
}

fn top_3(data: &[u8]) -> u64 {
    let mut cals = get_cals(data);
    cals.sort();
    cals.iter().rev().take(3).sum()
}

fn get_cals(data: &[u8]) -> Vec<u64> {
    let mut parts = Vec::with_capacity(512);
    let mut sum = 0u64;

    for line in data.lines() {
        if line.is_empty() {
            parts.push(sum);
            sum = 0;
        } else {
            match atoi::<u64>(line) {
                Some(n) => sum += n,
                _ => panic!("Invalid num '{}'", String::from_utf8_lossy(line)),
            }
        }
    }

    if sum > 0 {
        parts.push(sum);
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &[u8] = b"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        ";

    #[test]
    fn part_1_sample() {
        assert_eq!(24_000, max(DATA.trim()));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(45_000, top_3(DATA.trim()));
    }
}
