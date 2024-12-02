use std::io::stdin;

use nom::{
    character::complete::{space1, u8},
    multi::separated_list0,
    IResult,
};

fn main() {
    let (safe_reports, unsafe_reports): (Vec<Vec<u8>>, Vec<Vec<u8>>) = stdin()
        .lines()
        .map(|l| parse_line(&l.unwrap()).expect("malformed input").1)
        .partition(|r| is_safe(r));

    // part 1
    let safe_count = safe_reports.len();
    println!("{}", safe_count);

    // part 2
    let safe_with_dampener_count = unsafe_reports
        .iter()
        .filter(|&r| is_safe_with_dampener(r))
        .count();
    println!("{}", safe_count + safe_with_dampener_count);
}

fn parse_line(i: &str) -> IResult<&str, Vec<u8>> {
    separated_list0(space1, u8)(i)
}

fn is_safe(report: &[u8]) -> bool {
    report.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
        || report.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
}

fn is_safe_with_dampener(report: &[u8]) -> bool {
    (0..report.len()).any(|n| {
        is_safe(
            &report
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| if i == n { None } else { Some(e) })
                .collect::<Vec<_>>(),
        )
    })
}
