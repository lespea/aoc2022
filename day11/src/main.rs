extern crate core;

mod monkey;
mod parser;

use crate::monkey::Monkeys;
use color_eyre::eyre::Result;

const PART_1: bool = false;

static INPUT: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let (_, monkeys) = parser::parse_monkeys(INPUT)?;

    if PART_1 {
        part1(monkeys)
    } else {
        part2(monkeys)
    }
}

fn part1(mut monkeys: Monkeys) -> Result<()> {
    println!("{}", monkeys.run(20, 2, false));
    Ok(())
}

fn part2(mut monkeys: Monkeys) -> Result<()> {
    println!("{}", monkeys.run(10_000, 2, true));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_monkeys;

    #[test]
    fn example1() {
        let mut monkeys = example_monkeys();
        assert_eq!(4, monkeys.0.len());

        assert_eq!(10_605, monkeys.run(20, 2, false));
    }

    #[test]
    fn example2() {
        let mut monkeys = example_monkeys();
        assert_eq!(4, monkeys.0.len());

        assert_eq!(2_713_310_158, monkeys.run(10_000, 2, true));
    }

    fn example_monkeys() -> Monkeys {
        let (rem, monkeys) = parse_monkeys(example_data()).unwrap();
        println!("{rem}");
        assert!(rem.is_empty());
        monkeys
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
