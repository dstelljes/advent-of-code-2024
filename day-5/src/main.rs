use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
    io::{stdin, Read},
};

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u8},
    multi::{many1, separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let (_, (rules, updates)) = parse_input(&buffer).expect("malformed input");
    let comparator = comparator(&rules);

    let (valid, mut invalid): (Vec<_>, Vec<_>) = updates
        .into_iter()
        .partition(|u| u.is_sorted_by(predicate(&comparator)));

    // part 1
    let valid_mid_sum = valid.iter().fold(0, |sum, u| sum + u[u.len() / 2] as u32);
    println!("{}", valid_mid_sum);

    // part 2
    for update in invalid.iter_mut() {
        update.sort_by(&comparator);
    }

    let reordered_mid_sum = invalid.iter().fold(0, |sum, u| sum + u[u.len() / 2] as u32);
    println!("{}", reordered_mid_sum);
}

fn parse_rule(i: &str) -> IResult<&str, (u8, u8)> {
    separated_pair(u8, tag("|"), u8)(i)
}

fn parse_rules(i: &str) -> IResult<&str, HashMap<u8, HashSet<u8>>> {
    separated_list0(newline, parse_rule)(i).map(|(i, rules)| {
        (
            i,
            rules.iter().fold(HashMap::new(), |mut map, (a, b)| {
                map.entry(*a).or_insert_with(HashSet::new).insert(*b);
                map
            }),
        )
    })
}

fn parse_update(i: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(","), u8)(i)
}

fn parse_updates(i: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list0(newline, parse_update)(i)
}

fn parse_input(i: &str) -> IResult<&str, (HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>)> {
    let (i, rules) = parse_rules(i)?;
    let (i, _) = many1(newline)(i)?;
    let (i, updates) = parse_updates(i)?;

    Ok((i, (rules, updates)))
}

fn comparator<T>(rules: &HashMap<T, HashSet<T>>) -> impl Fn(&T, &T) -> Ordering + use<'_, T>
where
    T: Eq + Hash,
{
    move |a, b| {
        if rules.get(a).map_or(false, |s| s.contains(b)) {
            return Ordering::Less;
        }

        if rules.get(b).map_or(false, |s| s.contains(a)) {
            return Ordering::Greater;
        }

        return Ordering::Equal;
    }
}

fn predicate<T, F>(comparator: &F) -> impl Fn(&T, &T) -> bool + use<'_, T, F>
where
    T: Eq + Hash,
    F: Fn(&T, &T) -> Ordering,
{
    move |a, b| comparator(a, b) != Ordering::Greater
}
