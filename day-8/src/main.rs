use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    usize,
};

struct Map {
    antennas: HashMap<char, HashSet<Point>>,
    height: usize,
    width: usize,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(usize, usize);

fn main() {
    let map = parse_input(stdin().lines().map(|l| l.unwrap()));

    // part 1
    println!("{}", find_antinodes(&map, false).len());

    // part 2
    println!("{}", find_antinodes(&map, true).len());
}

fn parse_input(i: impl Iterator<Item = String>) -> Map {
    let mut map = Map {
        antennas: HashMap::new(),
        height: 0,
        width: 0,
    };

    for (y, line) in i.enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => {}
                _ => {
                    map.antennas
                        .entry(char)
                        .or_insert(HashSet::new())
                        .insert(Point(x, y));
                }
            }
        }

        if map.height == 0 {
            map.width = line.len();
        } else {
            assert_eq!(map.width, line.len());
        }

        map.height += 1;
    }

    map
}

fn find_antinodes(map: &Map, all: bool) -> HashSet<Point> {
    map.antennas
        .iter()
        .flat_map(|(_, antennas)| {
            antennas
                .iter()
                .flat_map(|a| antennas.iter().map(move |b| (a, b)))
                .filter(|&(a, b)| a != b)
                .flat_map(|(&Point(xa, ya), &Point(xb, yb))| {
                    if all { 0..=usize::MAX } else { 1..=1 }.map_while(move |i| {
                        let dx = (xb as isize - xa as isize) * (i as isize);
                        let dy = (yb as isize - ya as isize) * (i as isize);

                        let (xn, xo) = xb.overflowing_add_signed(dx);
                        let (yn, yo) = yb.overflowing_add_signed(dy);

                        if xo || xn >= map.width || yo || yn >= map.height {
                            None
                        } else {
                            Some(Point(xn, yn))
                        }
                    })
                })
        })
        .collect()
}
