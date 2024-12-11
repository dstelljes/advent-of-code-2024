use std::{
    collections::HashMap,
    io::{stdin, Read},
};

use nom::{
    character::complete::{space1, u64},
    multi::separated_list1,
    IResult,
};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let (_, mut stone_counts) = parse_input(&buffer).expect("malformed input");

    // part 1
    stone_counts = (0..25).fold(stone_counts, |acc, _| blink(acc));
    println!("{:?}", stone_counts.values().sum::<u64>());

    // part 2
    stone_counts = (25..75).fold(stone_counts, |acc, _| blink(acc));
    println!("{:?}", stone_counts.values().sum::<u64>());
}

fn parse_input(i: &str) -> IResult<&str, HashMap<u64, u64>> {
    separated_list1(space1, u64)(i).map(|(i, v)| {
        (
            i,
            v.into_iter().fold(HashMap::new(), |mut acc, n| {
                *acc.entry(n).or_default() += 1;
                acc
            }),
        )
    })
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    stones
        .into_iter()
        .fold(HashMap::new(), |mut acc, (n, count)| {
            if n == 0 {
                *acc.entry(1).or_default() += count;
            } else {
                let digits = n.checked_ilog10().unwrap_or(0) + 1;

                if digits % 2 == 0 {
                    let split = 10_u64.pow(digits / 2);
                    *acc.entry(n / split).or_default() += count;
                    *acc.entry(n % split).or_default() += count;
                } else {
                    *acc.entry(n * 2024).or_default() += count;
                }
            }

            acc
        })
}
