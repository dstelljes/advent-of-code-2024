use nom::{
    bytes::complete::tag,
    character::complete::{i64, space1, u64},
    sequence::tuple,
    IResult,
};
use std::{collections::HashSet, io::stdin};

#[derive(Eq, Hash, PartialEq)]
struct Point(u64, u64);

struct Velocity(i64, i64);

struct Robot {
    position: Point,
    velocity: Velocity,
}

fn main() {
    let width = 101;
    let height = 103;
    let robots = stdin()
        .lines()
        .map(|l| parse_line(&l.unwrap()).expect("malformed input").1)
        .collect::<Vec<_>>();

    // part 1
    let positions = robots.iter().map(|r| position_at(r, 100, width, height));
    let positions_by_quadrant = into_quadrants(positions, width, height);
    let safety_factor = positions_by_quadrant.iter().fold(1, |acc, q| acc * q.len());
    println!("{}", safety_factor);

    // part 2
    let tree_time = find_tree_time(&robots, width, height).expect("nope");
    println!("{}", tree_time);
}

fn parse_position(i: &str) -> IResult<&str, Point> {
    tuple((tag("p="), u64, tag(","), u64))(i).map(|(i, (_, x, _, y))| (i, Point(x, y)))
}

fn parse_velocity(i: &str) -> IResult<&str, Velocity> {
    tuple((tag("v="), i64, tag(","), i64))(i).map(|(i, (_, x, _, y))| (i, Velocity(x, y)))
}

fn parse_line(i: &str) -> IResult<&str, Robot> {
    tuple((parse_position, space1, parse_velocity))(i)
        .map(|(i, (position, _, velocity))| (i, Robot { position, velocity }))
}

fn position_at(
    &Robot {
        position: Point(x, y),
        velocity: Velocity(vx, vy),
    }: &Robot,
    t: i64,
    width: u64,
    height: u64,
) -> Point {
    let mut x = (x as i64 + (vx * t)) % width as i64;
    let mut y = (y as i64 + (vy * t)) % height as i64;

    if x < 0 {
        x += width as i64;
    }

    if y < 0 {
        y += height as i64;
    }

    Point(x as u64, y as u64)
}

fn into_quadrants(points: impl Iterator<Item = Point>, width: u64, height: u64) -> [Vec<Point>; 4] {
    let mut quadrants = [const { vec![] }; 4];

    for point in points {
        match point {
            Point(x, y) if x < width / 2 && y < height / 2 => quadrants[0].push(point),
            Point(x, y) if x > width / 2 && y < height / 2 => quadrants[1].push(point),
            Point(x, y) if x < width / 2 && y > height / 2 => quadrants[2].push(point),
            Point(x, y) if x > width / 2 && y > height / 2 => quadrants[3].push(point),
            _ => (),
        }
    }

    quadrants
}

fn find_tree_time(robots: &[Robot], width: u64, height: u64) -> Option<i64> {
    (1..).find(|&t| {
        robots
            .iter()
            .map(|r| position_at(r, t, width, height))
            .collect::<HashSet<_>>()
            .len()
            == robots.len()
    })
}
