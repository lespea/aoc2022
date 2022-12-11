extern crate core;

mod monkey;
mod parser;

use crate::monkey::Monkies;
use color_eyre::eyre::Result;

const PART_1: bool = false;

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let (_, monkies) = parser::parse_monkies(INPUT)?;

    if PART_1 {
        part1(monkies)
    } else {
        part2(monkies)
    }
}

fn part1(mut monkies: Monkies) -> Result<()> {
    println!("{}", monkies.run(20, 2));
    Ok(())
}

fn part2(mut monkies: Monkies) -> Result<()> {
    println!("{}", monkies.run(20, 2));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_monkies;

    #[test]
    fn example() {
        let mut monkies = example_monkies();
        println!("{monkies}");
        assert_eq!(4, monkies.0.len());
        assert_eq!(10_605, monkies.run(20, 2))
    }

    fn example_monkies() -> Monkies {
        let (rem, monkies) = parse_monkies(example_data()).unwrap();
        println!("{rem}");
        assert!(rem.is_empty());
        monkies
    }

    fn example_data() -> &'static str {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
    }
}
