use color_eyre::eyre::{ErrReport, Result};
use std::cmp::Ordering;

use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::{char, line_ending, multispace0, multispace1, space0};
use nom::combinator::{all_consuming, map};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

const PART_1: bool = true;

static DATA: &str = include_str!("input");

fn main() -> Result<()> {
    color_eyre::install()?;

    let _ = DATA.len();

    if PART_1 {
        part1()
    } else {
        part2()
    }
}

fn part1() -> Result<()> {
    let sum: usize = PacketPairs::try_from(DATA)?
        .packets
        .iter()
        .enumerate()
        .map(|(idx, pp)| if pp.cmp().is_le() { idx + 1 } else { 0 })
        .sum();

    dbg!(sum);

    Ok(())
}

fn part2() -> Result<()> {
    Ok(())
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PacketPairs {
    pub packets: Vec<PacketPair>,
}

impl TryFrom<&str> for PacketPairs {
    type Error = ErrReport;

    fn try_from(input: &str) -> std::result::Result<Self, Self::Error> {
        Ok(PacketPairs {
            packets: all_consuming(terminated(
                separated_list1(multispace1, PacketPair::nom),
                multispace0,
            ))(input)
            .map_err(|e| ErrReport::from(e.to_owned()))?
            .1,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PacketPair {
    p1: Packet,
    p2: Packet,
}

impl PacketPair {
    fn nom(i: &str) -> IResult<&str, PacketPair> {
        map(
            tuple((Packet::nom, preceded(line_ending, Packet::nom))),
            |(p1, p2)| PacketPair { p1, p2 },
        )(i)
    }

    fn cmp(&self) -> Ordering {
        self.p1.cmp(&self.p2)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

fn compare_diff(n: u8, l: &[Packet]) -> Ordering {
    use Packet::*;

    let ord = match l.first() {
        None => Ordering::Greater,

        Some(Num(n2)) => n.cmp(n2),

        Some(List(l2)) => {
            if l2.is_empty() {
                Ordering::Greater
            } else {
                let pn = Num(n);

                let cmp = l2.iter().map(|p2| pn.cmp(p2)).find(|c| c.is_ne());

                cmp.unwrap_or({
                    if l2.len() == 1 {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                })
            }
        }
    };

    if ord.is_eq() && l.len() > 1 {
        Ordering::Less
    } else {
        ord
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Packet::*;

        Some(match (self, other) {
            (Num(n1), Num(n2)) => n1.cmp(n2),
            (Num(n), List(l)) => compare_diff(*n, l),
            (List(l), Num(n)) => compare_diff(*n, l).reverse(),
            (List(l1), List(l2)) => {
                let cmp = l1
                    .iter()
                    .zip(l2)
                    .map(|(p1, p2)| p1.cmp(p2))
                    .find(|c| c.is_ne());

                cmp.unwrap_or_else(|| l1.len().cmp(&l2.len()))
            }
        })
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl TryFrom<&str> for Packet {
    type Error = ErrReport;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match Packet::nom(value) {
            Ok((_, packet)) => Ok(packet),
            Err(e) => Err(ErrReport::from(e.to_owned())),
        }
    }
}

impl Packet {
    fn nom(i: &str) -> IResult<&str, Self> {
        alt((
            delimited(
                char('['),
                map(
                    separated_list0(delimited(space0, char(','), space0), Packet::nom),
                    Packet::List,
                ),
                char(']'),
            ),
            map(complete::u8, Packet::Num),
        ))(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tests() -> Vec<(&'static str, Packet)> {
        vec![
            (
                "[[1] ,[2,3 , 4]]",
                Packet::List(vec![
                    Packet::List(vec![Packet::Num(1)]),
                    Packet::List(vec![Packet::Num(2), Packet::Num(3), Packet::Num(4)]),
                ]),
            ),
            (
                "[1 ,[2, [3] , 4]]",
                Packet::List(vec![
                    Packet::Num(1),
                    Packet::List(vec![
                        Packet::Num(2),
                        Packet::List(vec![Packet::Num(3)]),
                        Packet::Num(4),
                    ]),
                ]),
            ),
        ]
    }

    #[test]
    fn simple_str_parse() {
        for (line, want) in tests().into_iter() {
            // Individual parts
            let (rest, got) = Packet::nom(line).unwrap();
            assert_eq!(want, got);
            assert!(rest.is_empty());

            // Single func
            assert_eq!(want, line.try_into().unwrap())
        }
    }

    #[test]
    fn example() {
        for (l1, l2, want) in [
            ("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less),
            ("[[1],[2,3,4]]", "[[1],4]", Ordering::Less),
            ("[9]", "[[8,7,6]]", Ordering::Greater),
            ("[7]", "[[8,7,6]]", Ordering::Less),
            ("[8]", "[[8,7,6]]", Ordering::Less),
            ("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less),
            ("[7,7,7,7]", "[7,7,7]", Ordering::Greater),
            ("[]", "[3]", Ordering::Less),
            ("1", "[[[3]]]", Ordering::Less),
            ("1", "[[[9]]]", Ordering::Less),
            ("1", "[[[1]]]", Ordering::Equal),
            ("1", "[[[1, 1]]]", Ordering::Less),
            ("3", "[[[2]]]", Ordering::Greater),
            ("[[[]]]", "[[]]", Ordering::Greater),
            ("[[[]]]", "[[[]]]", Ordering::Equal),
            ("1", "1", Ordering::Equal),
            (
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                "[1,[2,[3,[4,[5,6,0]]]],8,9]",
                Ordering::Greater,
            ),
        ] {
            let p1: Packet = l1.try_into().unwrap();
            assert_eq!(want, p1.cmp(&l2.try_into().unwrap()));
        }
    }

    #[test]
    fn multi_parse() {
        let packets = PacketPairs::try_from(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]",
        )
        .unwrap()
        .packets;

        assert_eq!(packets.len(), 2);

        assert_eq!(
            packets.iter().map(|p| p.cmp()).collect::<Vec<_>>(),
            vec![Ordering::Less, Ordering::Less],
        );
    }
}
