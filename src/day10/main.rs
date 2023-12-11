use core::fmt;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum MazeTile {
    Empty,
    Start,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
}

#[derive(Debug)]
struct PipeMaze {
    tiles: Vec<Vec<MazeTile>>,
    start: (i64, i64),
    loop_coords: Vec<(i64, i64)>,
    actual_start_tile: MazeTile,
}

// -------------------------------------------------------------------------- //

impl Direction {
    fn values() -> &'static [Direction] {
        &[Self::North, Self::East, Self::South, Self::West]
    }

    fn offset_by(&self, xy: (i64, i64), by: i64) -> (i64, i64) {
        match *self {
            Self::North => (xy.0, xy.1 - by),
            Self::East => (xy.0 + by, xy.1),
            Self::South => (xy.0, xy.1 + by),
            Self::West => (xy.0 - by, xy.1),
        }
    }

    fn opposite(&self) -> Direction {
        match *self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl MazeTile {
    fn new(from: char) -> MazeTile {
        match from {
            '.' => Self::Empty,
            'S' => Self::Start,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BendNE,
            'J' => Self::BendNW,
            '7' => Self::BendSW,
            'F' => Self::BendSE,
            _ => unreachable!(),
        }
    }

    fn pipe_values() -> &'static [MazeTile] {
        &[  Self::Vertical, Self::Horizontal,
            Self::BendNE, Self::BendNW,
            Self::BendSW, Self::BendSE  ]
    }

    fn next_dir(&self, indir: Direction) -> Option<Direction> {
        match *self {
            Self::Vertical => match indir {
                Direction::North | Direction::South => Some(indir),
                _ => None,
            },
            Self::Horizontal => match indir {
                Direction::East | Direction::West => Some(indir),
                _ => None,
            },
            Self::BendNE => match indir {
                Direction::South => Some(Direction::East),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Self::BendNW => match indir {
                Direction::South => Some(Direction::West),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Self::BendSW => match indir {
                Direction::North => Some(Direction::West),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            Self::BendSE => match indir {
                Direction::North => Some(Direction::East),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            _ => None,
        }
    }
}

impl fmt::Display for MazeTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self { 
            Self::Empty => " ",
            Self::Start => "S",
            // Unicode box-drawing characters for prettier pipes,
            Self::Vertical => "│",
            Self::Horizontal => "─",
            Self::BendNE => "└",
            Self::BendNW => "┘",
            Self::BendSW => "┐",
            Self::BendSE => "┌",
        };
        write!(f, "{}", s)
    }
}

impl PipeMaze {
    fn new(tiles: Vec<Vec<MazeTile>>) -> PipeMaze {
        let mut start: Option<(i64, i64)> = None;

        for y in 0..tiles.len() {
            let line = &tiles[y];
            for x in 0..line.len() {
                if line[x] == MazeTile::Start {
                    assert!(start.is_none());
                    start = Some((x as i64, y as i64 ));
                }
            }
        }

        PipeMaze {
            tiles: tiles,
            start: start.unwrap(),
            loop_coords: Vec::new(),
            actual_start_tile: MazeTile::Start,
        }
    }

    fn at(&self, xy: (i64, i64)) -> Option<MazeTile> {
        if (xy.0 < 0) || (xy.1 < 0) {
            return None;
        }
        let x = xy.0 as usize;
        let y = xy.1 as usize;

        if y < self.tiles.len() {
            let line = &self.tiles[y];
            if x < line.len() {
                return Some(line[x]);
            }
        }
        None
    }

    fn adjacent(&self, xy: (i64, i64), dir: Direction) -> Option<MazeTile> {
        self.at(dir.offset_by(xy, 1))
    }

    fn is_valid_dir(&self, xy: (i64, i64), dir: Direction) -> bool {
        let a = self.adjacent(xy, dir);
        match a {
            Some(t) => t.next_dir(dir).is_some(),
            None => false,
        }
    }

    fn extract_loop_tiles(&mut self) {
        let mut pos = self.start;
        let mut dir = Direction::North;
        let mut coords: Vec<(i64, i64)> = Vec::new();

        let mut start_dir = Direction::North;
        let mut end_dir = Direction::North;

        assert_eq!(self.at(pos), Some(MazeTile::Start));
        for d in Direction::values() {
            if self.is_valid_dir(pos, *d) {
                dir = *d;
                start_dir = dir;
                break;
            }
        }

        coords.push(pos);
        loop {
            pos = dir.offset_by(pos, 1);
            let next = self.at(pos).unwrap();

            if next == MazeTile::Start {
                end_dir = dir;
                break;
            }

            dir = next.next_dir(dir).unwrap();
            coords.push(pos);
        }

        let coord_set: HashSet<(i64, i64)> = HashSet::from_iter(coords.clone());
        for y in 0..self.tiles.len() {
            let line = &mut self.tiles[y];
            for x in 0..line.len() {
                let c = (x as i64, y as i64);
                if !coord_set.contains(&c) {
                    line[x] = MazeTile::Empty;
                }
            }
        }

        for t in MazeTile::pipe_values() {
            let out_valid = t.next_dir(start_dir.opposite()).is_some();
            let in_valid = t.next_dir(end_dir).is_some();

            if out_valid && in_valid {
                self.actual_start_tile = *t;
            }
        }
        assert_ne!(self.actual_start_tile, MazeTile::Start);

        self.loop_coords = coords;
    }
}

impl fmt::Display for PipeMaze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = String::from("PipeMaze {\n");

        // Fancy display with added border,
        // NOTE: Requires terminal with codepage set to 65001 on Windows
        s += "  ┌";
        s += &"─".repeat(self.tiles[0].len());
        s += "┐\n";
    
        for line in &self.tiles {
            s += "  │";
            for tile in line {
                s += &format!("{}", tile);
            }
            s += "│\n";
        }

        s += "  └";
        s += &"─".repeat(self.tiles[0].len());
        s += "┘";
    
        write!(f, "{},\n}}", s)
    }
}

// -------------------------------------------------------------------------- //

fn part1(pipes: &PipeMaze) -> i64 {
    let loop_len = pipes.loop_coords.len() as i64;
    loop_len / 2
}

fn part2(pipes: &PipeMaze) -> i64 {
    let mut loop_min = pipes.loop_coords[0];
    let mut loop_max = loop_min;

    for c in &pipes.loop_coords {
        loop_min.0 = loop_min.0.min(c.0);
        loop_min.1 = loop_min.1.min(c.1);

        loop_max.0 = loop_max.0.max(c.0);
        loop_max.1 = loop_max.1.max(c.1);
    }

    let w = (loop_max.0 - loop_min.0 + 1) as usize;
    let h = (loop_max.1 - loop_min.1 + 1) as usize;
    let mut flags: Vec<Vec<u8>> = Vec::new();
    flags.resize(h, vec![0; w]);

    const TOP_EDGE: u8 = 0x1;
    const BOTTOM_EDGE: u8 = 0x2;

    for c in &pipes.loop_coords {
        let x = (c.0 - loop_min.0) as usize;
        let y = (c.1 - loop_min.1) as usize;
        let t = if *c != pipes.start {
            pipes.at(*c).unwrap()
        } else {
            pipes.actual_start_tile
        };

        flags[y][x] = match t {
            MazeTile::Vertical => (TOP_EDGE | BOTTOM_EDGE),
            MazeTile::Horizontal => 0,
            MazeTile::BendNE | MazeTile::BendNW => TOP_EDGE,
            MazeTile::BendSE | MazeTile::BendSW => BOTTOM_EDGE,
            _ => 0
        };
    }

    let mut inner_count = 0;
    for y in 0..h {
        let mut top_inside = false;
        let mut bottom_inside = false;

        for x in 0..w {
            let f = flags[y][x];
            let top = (f & TOP_EDGE) != 0;
            let bottom = (f & BOTTOM_EDGE) != 0;

            if (f & TOP_EDGE) != 0 {
                top_inside = !top_inside;
            }
            if (f & BOTTOM_EDGE) != 0 {
                bottom_inside = !bottom_inside;
            }

            if (f == 0) && top_inside && bottom_inside {
                inner_count += 1;
                flags[y][x] |= 0x8;
            }
        }
    }

    inner_count
}

fn main() {
    let start = Instant::now();

    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let lines = input.lines();
    let mut tiles: Vec<Vec<MazeTile>> = Vec::new();

    for line in lines {
        let l: Vec<MazeTile> = line.chars().map(|c| MazeTile::new(c)).collect();
        tiles.push(l);
    }

    let mut pipes = PipeMaze::new(tiles);
    pipes.extract_loop_tiles();
    println!("{}", pipes);

    let p1 = part1(&pipes);
    println!("Part 1 result: {}", p1);
    
    let p2 = part2(&pipes);
    println!("Part 2 result = {}", p2);

    let runtime = Instant::now() - start;
    println!("Completed in {} ms", runtime.as_nanos() as f64 / 1e6);
}
