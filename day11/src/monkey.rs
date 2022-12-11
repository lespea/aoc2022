use std::fmt::{Debug, Display, Formatter, Write};
use std::num::ParseIntError;

pub type ItemNum = u64;

const DEBUG: bool = false;
const DEBUG_MATH: bool = DEBUG && false;

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Debug, Default)]
pub struct Monkies(pub Vec<Monkey>);

impl Monkies {
    fn lcm(&self) -> ItemNum {
        self.0.iter().map(|m| m.test_mod).fold(1, num::integer::lcm)
    }

    pub fn run(&mut self, n: usize, top: usize, worry: bool) -> usize {
        let lcm = self.lcm();
        let mut moves: Vec<ItemMove> = Vec::with_capacity(128);

        for _ in 0..n {
            if DEBUG {
                println!("{self}\n");
            }
            for idx in 0..self.0.len() {
                self.0[idx].take_turn(&mut moves, lcm, worry);
                for ItemMove { item, target } in moves.drain(0..) {
                    self.0[target].items.push(item);
                }
            }
            if DEBUG {
                println!();
            }
        }

        if DEBUG {
            println!("{self}");
        }

        let mut all_inspected: Vec<usize> = self.0.iter().map(|m| m.num_inspected).collect();

        all_inspected.sort_unstable();

        all_inspected.into_iter().rev().take(top).product()
    }
}

impl Display for Monkies {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, monkey) in self.0.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }

            f.write_fmt(format_args!("Monkey {i}: ({:3})", monkey.num_inspected))?;

            for (idx, item) in monkey.items.iter().enumerate() {
                if idx != 0 {
                    f.write_char(',')?;
                }
                f.write_fmt(format_args!(" {}", item.0))?;
            }
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Debug)]
pub struct Monkey {
    pub num: u8,
    pub items: Vec<Item>,
    pub num_inspected: usize,
    pub op: Op,
    pub test_mod: ItemNum,
    pub target_true: usize,
    pub target_false: usize,
}

impl Monkey {
    pub fn take_turn(&mut self, targets: &mut Vec<ItemMove>, lcm: ItemNum, worry: bool) {
        if DEBUG {
            println!("{self}");
        }

        self.num_inspected += self.items.len();

        targets.extend(self.items.drain(..).map(|i| {
            let i = self.op.adjust(i, worry);

            let im = if i % self.test_mod == 0 {
                ItemMove {
                    item: Item(i % lcm),
                    target: self.target_true,
                }
            } else {
                ItemMove {
                    item: Item(i % lcm),
                    target: self.target_false,
                }
            };

            if DEBUG_MATH {
                println!("  => {}", im.target)
            }

            im
        }));
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Monkey");

        dbg.field("Items", &self.items);
        dbg.field("Inspected", &self.num_inspected);

        dbg.finish()
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Default)]
pub struct Item(pub ItemNum);

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl TryFrom<&str> for Item {
    type Error = ParseIntError;

    fn try_from(n: &str) -> Result<Self, Self::Error> {
        Ok(Item(n.parse()?))
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug, Default)]
pub struct ItemMove {
    item: Item,
    target: usize,
}

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug)]
pub enum Op {
    Add(ItemNum),
    Mult(ItemNum),
    Double,
}

impl TryFrom<(char, &str)> for Op {
    type Error = ParseIntError;

    fn try_from(t: (char, &str)) -> Result<Self, Self::Error> {
        use Op::*;

        Ok(match t.0 {
            '+' => Add(t.1.parse()?),

            '*' => {
                if t.1 == "old" {
                    Double
                } else {
                    Mult(t.1.parse()?)
                }
            }

            _ => panic!("Impossible"),
        })
    }
}

impl Op {
    pub fn adjust(self, old: Item, should_worry: bool) -> ItemNum {
        use Op::*;

        let worry = old.0;

        let worry = match self {
            Add(n) => worry + n,
            Mult(n) => worry * n,
            Double => worry * worry,
        };

        let ans = if should_worry {
            worry
        } else {
            (worry as f64 / 3.0).floor() as ItemNum
        };

        if DEBUG_MATH {
            println!("{} -> {worry} -> {ans}", old.0)
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        use Op::*;

        let mut monkies = Monkies(vec![
            Monkey {
                num: 0,
                items: vec![Item(79), Item(98)],
                num_inspected: 0,
                op: Mult(19),
                test_mod: 23,
                target_true: 2,
                target_false: 3,
            },
            Monkey {
                num: 1,
                items: vec![Item(54), Item(65), Item(75), Item(74)],
                num_inspected: 0,
                op: Add(6),
                test_mod: 19,
                target_true: 2,
                target_false: 0,
            },
            Monkey {
                num: 2,
                items: vec![Item(79), Item(60), Item(97)],
                num_inspected: 0,
                op: Double,
                test_mod: 13,
                target_true: 1,
                target_false: 3,
            },
            Monkey {
                num: 3,
                items: vec![Item(74)],
                num_inspected: 0,
                op: Add(3),
                test_mod: 17,
                target_true: 0,
                target_false: 1,
            },
        ]);

        assert_eq!(10_605, monkies.run(20, 2, false))
    }
}
