use std::{io::stdin, iter::zip};

use nom::{
    bytes::complete::tag,
    character::complete::{space1, u64},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

enum Op {
    Add,
    Mul,
    Concat,
}

struct Equation {
    value: u64,
    operands: Vec<u64>,
}

fn main() {
    let equations: Vec<Equation> = stdin()
        .lines()
        .map(|l| parse_line(&l.unwrap()).expect("malformed input").1)
        .collect();

    // part 1
    let add_mul_count = equations
        .iter()
        .filter(|&equation| is_possible(&[Op::Add, Op::Mul], equation))
        .fold(0, |acc, Equation { value, .. }| acc + value);
    println!("{}", add_mul_count);

    // part 2
    let add_mul_concat_count = equations
        .iter()
        .filter(|&equation| is_possible(&[Op::Add, Op::Mul, Op::Concat], equation))
        .fold(0, |acc, Equation { value, .. }| acc + value);
    println!("{}", add_mul_concat_count);
}

fn parse_line(i: &str) -> IResult<&str, Equation> {
    separated_pair(u64, tuple((tag(":"), space1)), separated_list1(space1, u64))(i)
        .map(|(i, (value, operands))| (i, Equation { value, operands }))
}

fn evaluate(operands: &[u64], operators: &[&Op]) -> u64 {
    let mut result = operands[0];

    for (operand, &op) in zip(operands.iter().skip(1), operators) {
        result = match op {
            Op::Add => result + operand,
            Op::Mul => result * operand,
            Op::Concat => {
                (result * 10_u64.pow(operand.checked_ilog10().unwrap_or(0) + 1)) + operand
            }
        }
    }

    result
}

fn is_possible(operators: &[Op], equation: &Equation) -> bool {
    permute(operators, equation.operands.len() - 1)
        .iter()
        .any(|operators| evaluate(&equation.operands, operators) == equation.value)
}

fn permute(operators: &[Op], count: usize) -> Vec<Vec<&Op>> {
    match count {
        0 => vec![],
        1 => operators.iter().map(|op| vec![op]).collect(),
        _ => {
            let mut result = vec![];

            for op in operators.iter() {
                let mut permutations = permute(operators, count - 1);

                for permutation in permutations.iter_mut() {
                    permutation.push(op);
                }

                result.append(&mut permutations);
            }

            result
        }
    }
}
