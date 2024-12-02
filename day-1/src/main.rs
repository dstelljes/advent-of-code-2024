use std::collections::HashMap;
use std::io::stdin;
use std::iter::zip;

use nom::character::complete::{space1, u32};
use nom::sequence::separated_pair;
use nom::IResult;

fn main() {
    let (mut firsts, mut seconds): (Vec<u32>, Vec<u32>) = stdin()
        .lines()
        .map(|l| parse_line(&l.unwrap()).expect("malformed input").1)
        .unzip();

    // part 1
    firsts.sort();
    seconds.sort();

    let distance_between_sorteds: u32 = zip(firsts.iter(), seconds.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum();
    println!("{}", distance_between_sorteds);

    // part 2
    let occurrence_counts = seconds.iter().fold(HashMap::new(), |mut acc, &id| {
        *acc.entry(id).or_insert(0) += 1;
        acc
    });

    let similarity_score: u32 = firsts
        .iter()
        .map(|&id| occurrence_counts.get(&id).map_or(0, |&count| count * id))
        .sum();
    println!("{}", similarity_score);
}

fn parse_line(i: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, space1, u32)(i)
}
