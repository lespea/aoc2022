use crate::monkey::Op;
use crate::monkey::{Item, Monkey};
use crate::monkey::{ItemNum, Monkies};

use nom::branch::alt;
use nom::character::complete::{
    alphanumeric1, digit1, line_ending, multispace1, one_of, space0, space1,
};
use nom::combinator::{eof, map};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::{
    bytes::complete::tag,
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, terminated},
    IResult,
};

pub fn parse_monkies(i: &str) -> IResult<&str, Monkies> {
    map(
        terminated(separated_list1(multispace1, parse_monkey), eof),
        Monkies,
    )(i)
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            parse_monkey_start,
            parse_items,
            parse_op,
            parse_mod,
            target_true,
            target_false,
        )),
        |(num, items, op, test_mod, target_true, target_false)| Monkey {
            num,
            items,
            num_inspected: 0,
            op,
            test_mod,
            target_true,
            target_false,
        },
    )(i)
}

fn to_u8(i: &str) -> Result<u8, std::num::ParseIntError> {
    i.parse()
}

fn parse_monkey_start(i: &str) -> IResult<&str, u8> {
    map_res(
        delimited(
            terminated(tag("Monkey"), space1),
            terminated(digit1, terminated(tag(":"), space0)),
            line_ending,
        ),
        to_u8,
    )(i)
}

fn parse_item(i: &str) -> IResult<&str, Item> {
    map_res(digit1, Item::try_from)(i)
}

fn parse_items(i: &str) -> IResult<&str, Vec<Item>> {
    delimited(
        delimited(space1, tag("Starting items:"), space0),
        separated_list0(delimited(space0, tag(","), space0), parse_item),
        line_ending,
    )(i)
}

fn parse_op(i: &str) -> IResult<&str, Op> {
    delimited(
        delimited(space1, tag("Operation: new = old"), space1),
        map_res(
            tuple((one_of("+*"), preceded(space1, alphanumeric1))),
            Op::try_from,
        ),
        line_ending,
    )(i)
}

fn to_inum(i: &str) -> Result<ItemNum, std::num::ParseIntError> {
    i.parse()
}

fn parse_mod(i: &str) -> IResult<&str, ItemNum> {
    delimited(
        delimited(space1, tag("Test: divisible by"), space1),
        map_res(digit1, to_inum),
        line_ending,
    )(i)
}

fn to_us(i: &str) -> Result<usize, std::num::ParseIntError> {
    i.parse()
}

fn target_true(i: &str) -> IResult<&str, usize> {
    delimited(
        delimited(space1, tag("If true: throw to monkey"), space1),
        map_res(digit1, to_us),
        line_ending,
    )(i)
}

fn target_false(i: &str) -> IResult<&str, usize> {
    delimited(
        delimited(space1, tag("If false: throw to monkey"), space1),
        map_res(digit1, to_us),
        alt((line_ending, eof)),
    )(i)
}

#[test]
fn test_lines() {
    let (_, test_monkey_n) = parse_monkey_start("Monkey 4:\n").unwrap();
    assert_eq!(4, test_monkey_n);

    let (_, test_items) = parse_items("  Starting items: 75, 94, 66\n").unwrap();
    assert_eq!(vec![Item(75), Item(94), Item(66)], test_items);

    let (_, test_op) = parse_op("  Operation: new = old + 1\n").unwrap();
    assert_eq!(Op::Add(1), test_op);

    let (_, test_mod) = parse_mod(" Test: divisible by 17\n").unwrap();
    assert_eq!(17, test_mod);

    let (_, test_true) = target_true("    If true: throw to monkey 6\n").unwrap();
    assert_eq!(6, test_true);

    let (_, test_false) = target_false("    If false: throw to monkey 1\n").unwrap();
    assert_eq!(1, test_false);
}
