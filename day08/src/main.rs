use std::collections::HashSet;

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
    println!("{}", solve(PART_1_DATA, 4));
}

fn part2() {
    println!("{}", solve(PART_1_DATA, 14));
}

fn solve(data: &str, group_size: usize) -> usize {
    let mut dupes = HashSet::with_capacity(group_size * 2);

    'WIND: for (idx, wind) in data.as_bytes().windows(group_size).enumerate() {
        dupes.clear();

        for b in wind.iter() {
            if !dupes.insert(*b) {
                continue 'WIND;
            }
        }

        return idx + group_size;
    }
    panic!("no answer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        for (data, len, want) in [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 4, 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11),
        ] {
            assert_eq!(want, solve(data, len));
        }
    }
}
