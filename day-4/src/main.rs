use std::{collections::HashMap, io::stdin, ops::Index};

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<char>,
}

impl Index<&(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, (x, y): &(usize, usize)) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

fn main() {
    let grid = parse_grid(stdin().lines().map(|l| l.unwrap()));

    // part 1
    println!("{}", count_xmas(&grid));

    // part 2
    println!("{}", count_x_mas(&grid));
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

fn count_xmas(grid: &Grid) -> usize {
    let directions = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let xmas = &['X', 'M', 'A', 'S'];

    let mut count = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            for direction in directions.iter() {
                if let Some(_) = find(grid, &(x, y), direction, xmas) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_x_mas(grid: &Grid) -> usize {
    let directions = &[(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mas = &['M', 'A', 'S'];

    let mut counts = HashMap::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            for direction in directions.iter() {
                if let Some([_, a, _]) = find(grid, &(x, y), direction, mas) {
                    *counts.entry(a).or_insert(0) += 1;
                }
            }
        }
    }

    counts.iter().filter(|(_, &count)| count > 1).count()
}

fn find<const N: usize>(
    grid: &Grid,
    origin: &(usize, usize),
    step: &(isize, isize),
    chars: &[char; N],
) -> Option<[(usize, usize); N]> {
    let mut path = [origin.to_owned(); N];

    for index in 0..N {
        let position = &mut path[index];

        if index > 0 {
            let (x, y) = position;
            let (dx, dy) = step;

            let (xo, yo);
            (*x, xo) = x.overflowing_add_signed(dx * index as isize);
            (*y, yo) = y.overflowing_add_signed(dy * index as isize);

            if xo || *x >= grid.width || yo || *y >= grid.height {
                return None;
            }
        }

        if grid[position] != chars[index] {
            return None;
        }
    }

    Some(path)
}
