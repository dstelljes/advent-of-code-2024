use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    ops::Index,
};

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point(usize, usize);

impl Index<&Point> for Grid {
    type Output = u8;

    fn index(&self, &Point(x, y): &Point) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

fn main() {
    let grid = parse_grid(stdin().lines().map(|l| l.unwrap()));

    let trailheads = (0..grid.height)
        .flat_map(|y| (0..grid.width).map(move |x| Point(x, y)))
        .filter(|p| grid[p] == 0)
        .map(|p| (p.clone(), find_ends(&grid, &p, 0)))
        .collect::<HashMap<_, _>>();

    // part 1
    let score_sum = trailheads
        .values()
        .fold(0, |sum, v| sum + v.iter().collect::<HashSet<_>>().len());
    println!("{}", score_sum);

    // part 2
    let rating_sum = trailheads.values().fold(0, |sum, v| sum + v.len());
    println!("{}", rating_sum);
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
        grid.cells
            .extend(line.chars().map(|c| c.to_digit(10).map_or(10, |d| d as u8)));
    }

    grid
}

fn find_ends(grid: &Grid, position: &Point, elevation: u8) -> Vec<Point> {
    if grid[position] != elevation {
        vec![]
    } else if elevation == 9 {
        vec![position.clone()]
    } else {
        (&[(0, -1), (0, 1), (-1, 0), (1, 0)])
            .iter()
            .flat_map(|&(dx, dy)| {
                let &Point(x, y) = position;

                let (xn, xo) = x.overflowing_add_signed(dx);
                let (yn, yo) = y.overflowing_add_signed(dy);

                if xo || xn >= grid.width || yo || yn >= grid.height {
                    vec![]
                } else {
                    find_ends(grid, &Point(xn, yn), elevation + 1)
                }
            })
            .collect()
    }
}
