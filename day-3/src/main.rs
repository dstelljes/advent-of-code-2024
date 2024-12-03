use std::io::stdin;

use nom::{branch::alt, bytes::complete::tag, character::complete::u32, sequence::tuple, IResult};

enum Op {
    Do,
    Dont,
    Mul(u32, u32),
}

fn main() {
    let ops: Vec<Op> = stdin()
        .lines()
        .flat_map(|l| parse_line(&l.unwrap()).expect("malformed input").1)
        .collect();

    // part 1
    let sum_muls_only = ops.iter().fold(0, |sum, op| match op {
        Op::Mul(a, b) => sum + (a * b),
        _ => sum,
    });
    println!("{}", sum_muls_only);

    // part 2
    let (_, sum_all) = ops.iter().fold((true, 0), |(enabled, sum), op| match op {
        Op::Do => (true, sum),
        Op::Dont => (false, sum),
        Op::Mul(a, b) => (enabled, sum + if enabled { a * b } else { 0 }),
    });
    println!("{}", sum_all);
}

fn parse_do(i: &str) -> IResult<&str, Op> {
    tag("do()")(i).map(|(i, _)| (i, Op::Do))
}

fn parse_dont(i: &str) -> IResult<&str, Op> {
    tag("don't()")(i).map(|(i, _)| (i, Op::Dont))
}

fn parse_mul(i: &str) -> IResult<&str, Op> {
    tuple((tag("mul("), u32, tag(","), u32, tag(")")))(i)
        .map(|(i, (_, a, _, b, _))| (i, Op::Mul(a, b)))
}

fn parse_op(i: &str) -> IResult<&str, Op> {
    alt((parse_do, parse_dont, parse_mul))(i)
}

fn parse_line(i: &str) -> IResult<&str, Vec<Op>> {
    // feels like there should be a better way to do this?
    let vec = (0..i.len())
        .filter_map(|l| parse_op(&i[l..]).map(|(_, op)| op).ok())
        .collect();

    Ok((i, vec))
}
