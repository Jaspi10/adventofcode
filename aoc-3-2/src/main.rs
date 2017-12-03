use std::io::{stdin, BufRead, BufReader};
use std::ops::{Index, IndexMut};
use std::fmt;

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn n(self) -> Point {
        Point {
            x: self.x,
            y: self.y-1,
        }
    }

    fn e(self) -> Point {
        Point {
            x: self.x+1,
            y: self.y,
        }
    }

    fn s(self) -> Point {
        Point {
            x: self.x,
            y: self.y+1,
        }
    }

    fn w(self) -> Point {
        Point {
            x: self.x-1,
            y: self.y,
        }
    }
}

struct Grid {
    length: usize,
    grid: Vec<Vec<u32>>
}

impl Grid {
    fn new(mut length: usize) -> Grid {
        if length % 2 != 0 {
            length += 1;
        }

        let vec = vec![vec![0u32; length]; length];

        Grid {
            length: length,
            grid: vec,
        }
    }

    fn get_sum_around(&self, point: Point) -> u32 {
        let mut res = 0;

        res += self[point.n()];
        res += self[point.e()];
        res += self[point.s()];
        res += self[point.w()];

        res += self[point.n().e()];
        res += self[point.n().w()];
        res += self[point.s().e()];
        res += self[point.s().w()];

        res
    }
}

impl Index<Point> for Grid {
    type Output = u32;

    fn index(&self, index: Point) -> &u32 {
        let center = (self.length / 2) + 1;

        &self.grid[(center as isize + index.x) as usize][(center as isize + index.y) as usize]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut u32 {
        let center = (self.length / 2) + 1;

        &mut self.grid[(center as isize + index.x) as usize][(center as isize + index.y) as usize]
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.length {
            for j in 0..self.length {
                write!(f, "{:>6}, ", self.grid[j][i])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let br = BufReader::new(stdin());

    let input = br.lines().next().unwrap().unwrap().trim().parse::<u32>().unwrap();

    let mut grid = Grid::new(20);
    let mut point = Point {
        x: 0,
        y: 0,
    };
    grid[point] = 1;
    let mut distance = 1;

    'outer: loop {
        for _ in 0..distance {
            point = point.e();
            grid[point] = grid.get_sum_around(point);

            if grid[point] > input {
                break 'outer;
            }
        }

        for _ in 0..distance {
            point = point.n();
            grid[point] = grid.get_sum_around(point);

            if grid[point] > input {
                break 'outer;
            }
        }

        distance += 1;

        for _ in 0..distance {
            point = point.w();
            grid[point] = grid.get_sum_around(point);

            if grid[point] > input {
                break 'outer;
            }
        }

        for _ in 0..distance {
            point = point.s();
            grid[point] = grid.get_sum_around(point);

            if grid[point] > input {
                break 'outer;
            }
        }

        distance += 1;
    }
    println!("{:?}", grid);
    println!("grid[{}, {}] = {}", point.x, point.y, grid[point]);
}
