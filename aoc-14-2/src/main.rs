#![feature(inclusive_range_syntax)]

use std::io::{stdin, BufRead, BufReader};
use std::ops::{Index, IndexMut};
use std::fmt;

struct CircularList {
    data: Vec<u8>,
    length: usize,
}

impl CircularList {
    fn new(length: usize) -> CircularList {
        CircularList {
            data: (0..=(length-1) as u8).collect(),
            length: length,
        }
    }

    fn reverse_sublist(&mut self, start: usize, length: usize) {
        for i in 0..length/2 {
            self.swap(start+i, (start+length-1)-i); 
        }
    }

    fn swap(&mut self, mut a: usize, mut b: usize) {
        a = a % self.length;
        b = b % self.length;

        let tmp = self.data[a];
        self.data[a] = self.data[b];
        self.data[b] = tmp; 
    }
}

struct KnotHash {
    state: CircularList,
    position: usize,
    skip: usize,
}

impl KnotHash {
    fn new() -> KnotHash {
        KnotHash {
            state: CircularList::new(256),
            position: 0,
            skip: 0,
        }
    }

    fn calculate_hash(input: String) -> Vec<u8> {
        let suffix: [u8; 5] = [17, 31, 73, 47, 23];
        let lengths = input.as_bytes().into_iter().chain(suffix.into_iter()).map(|a| *a).collect::<Vec<u8>>();

        let mut hash = KnotHash::new();
        for _ in 0..64 {
            hash.do_round(lengths.as_slice());
        }

        hash.get_dense_hash()
    }

    fn get_dense_hash(&self) -> Vec<u8>{
        let mut ret = Vec::new();

        for i in 0..16 {
            let mut block = 0_u8;
            for j in (i*16)..(i*16)+16 {
                block = block ^ self.state.data[j];
            }
            ret.push(block);
        }

        ret
    }

    fn do_round(&mut self, lengths: &[u8]) {
        for length in lengths {
            self.state.reverse_sublist(self.position, *length as usize);
            self.position += (*length as usize) + self.skip;

            self.skip += 1;
        }
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn n(self) -> Point {
        let y = if self.y == 0{
            0
        } else {
            self.y-1
        };

        Point {
            x: self.x,
            y: y,
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
        let x = if self.x == 0{
            0
        } else {
            self.x-1
        };

        Point {
            x: x,
            y: self.y,
        }
    }
}

#[derive(Clone, Copy)]
enum Region {
    Unused,
    Unmarked,
    Marked(u16),
}

impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Region::Unused => write!(f, "."),
            &Region::Unmarked => write!(f, "#"),
            &Region::Marked(n) => write!(f, "{}", n),
        }
    }
}

struct Grid {
    length: usize,
    grid: Vec<Vec<Region>>
}

impl Grid {
    fn new(length: usize) -> Grid {
        let vec = vec![vec![Region::Unused; length]; length];

        Grid {
            length: length,
            grid: vec,
        }
    }
}

impl Index<Point> for Grid {
    type Output = Region;

    fn index(&self, index: Point) -> &Region {
        &self.grid[index.x][index.y]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut Region {
        &mut self.grid[index.x][index.y]
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.length {
            for j in 0..self.length {
                write!(f, "{:?}", self.grid[j][i])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let br = BufReader::new(stdin());
    
    let input = br.lines().next().unwrap().unwrap();

    let mut grid = Grid::new(128);

    println!("{:?}", grid);

    for row in 0..128 {
        let row_hash = KnotHash::calculate_hash(format!("{}-{}", input, row));
        for segment in 0..row_hash.len() {
            for bit_index in 0..8 {
                let bit = (row_hash[segment] & (1 << (7 - bit_index))) != 0;

                if bit {
                    grid[Point {x: segment*8 + bit_index, y: row}] = Region::Unmarked;
                }
            }
        } 
    }

    println!("");
    println!("");
    println!("");

    println!("{:?}", grid);

    let mut color = 0;
    let mut queue = Vec::new();

    for i in 0..128 {
        for j in 0..128 {
            let point = Point {x: j, y: i};
            
            if let Region::Unmarked = grid[point] {
                queue.push(point);

                while !queue.is_empty() {
                    let current_pos = queue.pop().unwrap();
                    println!("{}, {}", current_pos.x, current_pos.y);

                    grid[current_pos] = Region::Marked(color);

                    if current_pos.y < 127 {
                        if let Region::Unmarked = grid[current_pos.s()] {
                            queue.push(current_pos.s());
                        }
                    }

                    if let Region::Unmarked = grid[current_pos.n()] {
                        queue.push(current_pos.n());
                    }

                    if let Region::Unmarked = grid[current_pos.w()] {
                        queue.push(current_pos.w());
                    }

                    if current_pos.x < 127 {
                        if let Region::Unmarked = grid[current_pos.e()] {
                            queue.push(current_pos.e());
                        } 
                    }
                }
                
                color += 1;
            }
        }
    }

    println!("");
    println!("");
    println!("");

    println!("{:?}", grid);

    println!("{}", color);

}
