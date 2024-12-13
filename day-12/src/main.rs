use std::{
    collections::{HashSet, VecDeque},
    io::stdin,
    iter::zip,
    ops::Index,
};

#[derive(Eq, Hash, PartialEq)]
struct Point(usize, usize);

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<char>,
}

struct Region {
    plant: char,
    points: HashSet<Point>,
}

static DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let grid = parse_grid(stdin().lines().map(|l| l.unwrap()));
    let regions = find_regions(&grid);

    // part 1
    let perimeters_cost = regions.iter().fold(0, |c, r| c + (area(r) * perimeter(r)));
    println!("{}", perimeters_cost);

    // part 2
    let sides_cost = regions.iter().fold(0, |c, r| c + (area(r) * sides(r)));
    println!("{}", sides_cost);
}

impl Index<&Point> for Grid {
    type Output = char;

    fn index(&self, &Point(x, y): &Point) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

impl Grid {
    fn translate(&self, &Point(x, y): &Point, &(dx, dy): &(isize, isize)) -> Option<Point> {
        let (xn, xo) = x.overflowing_add_signed(dx);
        let (yn, yo) = y.overflowing_add_signed(dy);

        if xo || xn >= self.width || yo || yn >= self.height {
            None
        } else {
            Some(Point(xn, yn))
        }
    }
}

impl Region {
    fn translate(&self, &Point(x, y): &Point, &(dx, dy): &(isize, isize)) -> Option<Point> {
        let (xn, xo) = x.overflowing_add_signed(dx);
        let (yn, yo) = y.overflowing_add_signed(dy);

        if xo || yo {
            return None;
        }

        let point = Point(xn, yn);

        if self.points.contains(&point) {
            Some(point)
        } else {
            None
        }
    }
}

fn parse_grid(lines: impl Iterator<Item = String>) -> Grid {
    let mut grid = Grid {
        width: 0,
        height: 0,
        cells: vec![],
    };

    for line in lines {
        if grid.width == 0 {
            grid.width = line.len();
        } else {
            assert_eq!(grid.width, line.len(), "inconsistent line length");
        }

        grid.height += 1;
        grid.cells.extend(line.chars());
    }

    grid
}

fn find_regions(grid: &Grid) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point(x, y);

            if regions.iter().any(|r| r.points.contains(&point)) {
                continue;
            }

            let mut region = Region {
                plant: grid[&point],
                points: HashSet::new(),
            };

            let mut neighbors = VecDeque::from([point]);

            while let Some(position) = neighbors.pop_front() {
                if grid[&position] != region.plant || region.points.contains(&position) {
                    continue;
                }

                for delta in DIRECTIONS.iter() {
                    if let Some(neighbor) = grid.translate(&position, delta) {
                        neighbors.push_back(neighbor);
                    }
                }

                region.points.insert(position);
            }

            regions.push(region);
        }
    }

    regions
}

fn area(region: &Region) -> usize {
    region.points.len()
}

fn perimeter(region: &Region) -> usize {
    region.points.iter().fold(0, |sum, point| {
        sum + DIRECTIONS
            .iter()
            .filter(|&delta| region.translate(point, delta).is_none())
            .count()
    })
}

fn sides(region: &Region) -> usize {
    region.points.iter().fold(0, |sum, point| {
        sum + zip(DIRECTIONS.iter(), DIRECTIONS.iter().cycle().skip(1))
            .filter(|&(d1, d2)| {
                let n1 = region.translate(point, d1);
                let n2 = region.translate(point, d2);
                let n3 = region.translate(point, &(d1.0 + d2.0, d1.1 + d2.1));

                let exterior = n1.is_none() && n2.is_none();
                let interior = n1.is_some() && n2.is_some() && n3.is_none();
                exterior || interior
            })
            .count()
    })
}
