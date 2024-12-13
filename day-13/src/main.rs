use std::{
    io::{stdin, Read},
    ops::MulAssign,
};

use nom::{
    bytes::complete::tag,
    character::complete::{char, i64, newline},
    multi::{many0, separated_list0},
    sequence::tuple,
    IResult,
};

struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

struct Point(i64, i64);

impl MulAssign<i64> for Point {
    fn mul_assign(&mut self, rhs: i64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let (_, mut machines) = parse_input(&buffer).expect("malformed input");

    // part 1
    let total_cost = machines.iter().filter_map(solve).sum::<i64>();
    println!("{}", total_cost);

    // part 2
    for machine in machines.iter_mut() {
        machine.prize *= 10000000000000;
    }

    let total_cost_ridiculous = machines.iter().filter_map(solve).sum::<i64>();
    println!("{}", total_cost_ridiculous);
}

fn parse_button(c: char) -> impl Fn(&str) -> IResult<&str, Point> {
    move |i| {
        tuple((tag("Button "), char(c), tag(": X"), i64, tag(", Y"), i64))(i)
            .map(|(i, (_, _, _, x, _, y))| (i, Point(x, y)))
    }
}

fn parse_prize(i: &str) -> IResult<&str, Point> {
    tuple((tag("Prize: X="), i64, tag(", Y="), i64))(i).map(|(i, (_, x, _, y))| (i, Point(x, y)))
}

fn parse_machine(i: &str) -> IResult<&str, Machine> {
    let (i, a) = parse_button('A')(i)?;
    let (i, _) = newline(i)?;
    let (i, b) = parse_button('B')(i)?;
    let (i, _) = newline(i)?;
    let (i, prize) = parse_prize(i)?;

    Ok((i, Machine { a, b, prize }))
}

fn parse_input(i: &str) -> IResult<&str, Vec<Machine>> {
    separated_list0(many0(newline), parse_machine)(i)
}

fn solve(
    &Machine {
        a: Point(ax, ay),
        b: Point(bx, by),
        prize: Point(px, py),
    }: &Machine,
) -> Option<i64> {
    let d = (ax * by) - (ay * bx);
    let a = ((px * by) - (py * bx)) / d;
    let b = ((ax * py) - (ay * px)) / d;

    let x = (a * ax) + (b * bx);
    let y = (a * ay) + (b * by);

    if x == px && y == py {
        Some((a * 3) + b)
    } else {
        None
    }
}
