use std::collections::HashSet;
use std::env;
use std::fs;
use std::str;

struct Rect {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl Rect {
    fn grow(&mut self, by: usize, bounds: (usize, usize)) {
        self.right = {
            let r = self.right + by;
            if r < bounds.0 { r } else { bounds.0 }
        };
        self.bottom = {
            let b = self.bottom + by;
            if b < bounds.1 { b } else { bounds.1 }
        };
        self.top = if self.top > by { self.top - by } else { 0 };
        self.left = if self.left > by { self.left - by } else { 0 };
    }
}

#[derive(Debug)]
struct ByteMatrix {
    bytes: Vec<Vec<u8>>
}

impl ByteMatrix {
    fn new(width: usize, height: usize) -> ByteMatrix {
        ByteMatrix {
            bytes: vec![vec![0u8; width]; height]
        }
    }

    fn width(&self) -> usize { self.bytes.len() }

    fn height(&self) -> usize { self.bytes[0].len() }

    fn at(&self, x: usize, y: usize) -> u8 { self.bytes[y][x] }

    fn row(&self, row: usize) -> &Vec<u8> { &self.bytes[row] }

    fn row_mut(&mut self, row: usize) -> &mut Vec<u8> { &mut self.bytes[row] }

    fn parse_num_at(&self, x: usize, y: usize) -> Option<u32> {
        let row = self.row(y);
        let start = x;

        if !row[start].is_ascii_digit() {
            return None;
        }
        let mut iter = row[start..].iter();
        let end = {
            let pos = iter.position(|c| !c.is_ascii_digit());
            match pos {
                Some(p) => start + p,
                None => self.width(),
            }
        };
        let as_str = str::from_utf8(&row[start..end]).unwrap();
        let num = u32::from_str_radix(as_str, 10);
        match num {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }

    fn for_each_in_rect<F>(&self, r: &Rect, mut func: F)
        where F: FnMut(u8, usize, usize)
    {
        for y in r.top..r.bottom {
            for x in r.left..r.right {
                func(self.at(x, y), x, y)
            }
        }
    }
}

// -------------------------------------------------------------------------- //

fn part1(matrix: &ByteMatrix, num_ranges: &Vec<Vec<(usize, usize)>>) -> u32 {
    let width = matrix.width();
    let height = matrix.height();
    let mut sum = 0u32;

    for y in 0..matrix.height() {
        let row = matrix.row(y);
        let row_nums = &num_ranges[y];
 
        for (start, end) in row_nums {
            let rect = {
                let mut r = Rect {
                    top: y, bottom: y + 1, left: *start, right: *end + 1 
                };
                r.grow(1, (width, height));
                r
            };

            let mut valid_part_num = false;
            matrix.for_each_in_rect(&rect, |c, _, _| {
                if c.is_ascii_punctuation() && c != b'.' {
                    valid_part_num = true;
                }
            });
            if valid_part_num {
                let n = matrix.parse_num_at(*start, y).unwrap();
                sum += n;
            }
        }
    }
    sum
}

fn part2(matrix: &ByteMatrix, num_ranges: &Vec<Vec<(usize, usize)>>) -> u32 {
    let width = matrix.width();
    let height = matrix.height();
    let mut sum = 0u32;

    for y in 0..height {
        let row = matrix.row(y);

        for x in 0..row.len() {
            let c = row[x];
            let rect = {
                let mut r = Rect {
                    top: y, bottom: y + 1, left: x, right: x + 1
                };
                r.grow(1, (width, height));
                r
            };
            if c == b'*' {
                let mut overlapping: HashSet<(usize, usize)> = HashSet::new();
                matrix.for_each_in_rect(&rect, |c, x, y| {
                    let mut iter = num_ranges[y].iter();
                    let contained = iter.find(
                        |(s, e)| x >= *s && x <= *e
                    );
                    if contained.is_some() {
                        overlapping.insert((contained.unwrap().0, y));
                    }
                });

                if overlapping.len() != 2 {
                    continue;
                }

                let mut n = 1u32;
                for (x, y) in overlapping {
                    n *= matrix.parse_num_at(x, y).unwrap();
                }

                sum += n;
            }
        }
    }
    sum
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let width = 140;
    let height = 140;
    let mut matrix = ByteMatrix::new(width, height);

    let mut row_idx: usize = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        assert_eq!(bytes.len(), width);

        let row = matrix.row_mut(row_idx);
        row.copy_from_slice(bytes);

        row_idx += 1;
    }

    let mut num_ranges: Vec<Vec<(usize, usize)>> = Vec::new();
    num_ranges.resize(height, Default::default());

    for y in 0..height {
        let row = matrix.row(y);
        let row_nums = &mut num_ranges[y];
        let mut in_number = false;

        for i in 0..row.len() {
            let c = row[i];
            if c.is_ascii_digit() {
                if in_number {
                    row_nums.last_mut().unwrap().1 = i;
                }
                if !in_number {
                    in_number = true;
                    row_nums.push((i, i));
                }
            } else {
                in_number = false;
            }
        }
    }

    let p1_sum = part1(&matrix, &num_ranges);
    let p2_sum = part2(&matrix, &num_ranges);

    println!("part 1 sum = {}", p1_sum);
    println!("part 2 sum = {}", p2_sum);
}
