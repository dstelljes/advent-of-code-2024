use std::{collections::HashSet, io::stdin};

#[derive(Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Step {
    Leave,
    Move(Point),
    Turn(Direction),
}

#[derive(Clone)]
struct Map {
    height: usize,
    obstacles: HashSet<Point>,
    width: usize,
}

struct Path {
    is_loop: bool,
    points: HashSet<Point>,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(usize, usize);

fn main() {
    let (map, start) = parse_input(stdin().lines().map(|l| l.unwrap()));

    // part 1
    let path = walk(&map, start, Direction::Up);
    let distinct_point_count = path.points.len();
    println!("{}", distinct_point_count);

    // part 2
    let loop_possibility_count = path
        .points
        .iter()
        .filter(|&p| walk(&place_obstacle(&map, p), start, Direction::Up).is_loop)
        .count();
    println!("{}", loop_possibility_count);
}

fn parse_input(i: impl Iterator<Item = String>) -> (Map, Point) {
    let mut map = Map {
        height: 0,
        obstacles: HashSet::new(),
        width: 0,
    };

    let mut start = None;

    for (y, line) in i.enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    map.obstacles.insert(Point(x, y));
                }
                '^' => {
                    start = Some(Point(x, y));
                }
                _ => {}
            }
        }

        if map.height == 0 {
            map.width = line.len();
        } else {
            assert_eq!(map.width, line.len());
        }

        map.height += 1;
    }

    (map, start.expect("no start found"))
}

fn step(map: &Map, position: &Point, direction: &Direction) -> Step {
    let &Point(x, y) = position;
    let (dx, dy) = match direction {
        Direction::Up => (0, -1),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
    };

    let (xn, xo) = x.overflowing_add_signed(dx);
    let (yn, yo) = y.overflowing_add_signed(dy);

    if xo || xn >= map.width || yo || yn >= map.height {
        Step::Leave
    } else if map.obstacles.contains(&Point(xn, yn)) {
        Step::Turn(match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        })
    } else {
        Step::Move(Point(xn, yn))
    }
}

fn walk(map: &Map, mut position: Point, mut direction: Direction) -> Path {
    let mut path = Path {
        is_loop: false,
        points: HashSet::new(),
    };

    let mut turns = HashSet::new();

    loop {
        path.points.insert(position);

        match step(map, &position, &direction) {
            Step::Leave => break,
            Step::Move(next) => position = next,
            Step::Turn(next) => {
                if !turns.insert((position, direction)) {
                    path.is_loop = true;
                    break;
                }

                direction = next;
            }
        }
    }

    path
}

fn place_obstacle(map: &Map, obstacle: &Point) -> Map {
    let mut clone = map.clone();
    clone.obstacles.insert(*obstacle);
    clone
}
