use core::fmt;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::Index;
use std::time::Instant;


#[derive(Debug)]
struct GalaxyMap {
    galaxies: Vec<(i64, i64)>,
}

// -------------------------------------------------------------------------- //

impl GalaxyMap {
    fn new(map: &Vec<Vec<u8>>, empty_space_expansion: i64) -> GalaxyMap {
        let h = map.len();
        let w = map[0].len();

        let mut galaxies = vec![(0i64, 0i64); 0];
        let mut empty_rows = vec![0usize; 0];
        let mut empty_cols = vec![true; w];

        for y in 0..h {
            let mut row_empty = true;
            for x in 0..w {
                if map[y][x] == b'#' {
                    galaxies.push((x as i64, y as i64));
                    row_empty = false;
                    empty_cols[x] = false;
                }
            }
            if row_empty {
                empty_rows.push(y);
            }
        }

        for i in empty_rows {
            for g in &mut galaxies {
                if g.1 < (i as i64) {
                    g.1 -= empty_space_expansion;
                }
            }
        }
        for (i, e) in empty_cols.iter().enumerate() {
            if *e {
                for g in &mut galaxies {
                    if g.0 < (i as i64) {
                        g.0 -= empty_space_expansion;
                    }
                }
            }
        }

        GalaxyMap {
            galaxies: galaxies,
        }
    }
}

// -------------------------------------------------------------------------- //

fn manhattan_dist(a: (i64, i64), b: (i64, i64)) -> i64 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

fn sum_galaxy_distances(map: &GalaxyMap) -> i64 {
    let mut sum = 0;
    
    for i in 0..map.galaxies.len() {
        let a = &map.galaxies[i];
        for b in &map.galaxies[(i+1)..] {
            let dist = manhattan_dist(*a, *b);
            sum += dist;
        }
    }

    sum
}

fn main() {
    let start = Instant::now();

    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let lines: std::str::Lines<'_> = input.lines();
    let mut tiles: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        tiles.push(line.chars().map(|c| c as u8).collect());
    }

    const PART1_EXPANSION: i64 = 1;
    const PART2_EXPANSION: i64 = 999_999;

    let p1_galaxy_map = GalaxyMap::new(&tiles, PART1_EXPANSION);
    let p2_galaxy_map = GalaxyMap::new(&tiles, PART2_EXPANSION);

    let p1 = sum_galaxy_distances(&p1_galaxy_map);
    println!("Part 1 result: {}", p1);
    
    let p2 = sum_galaxy_distances(&p2_galaxy_map);
    println!("Part 2 result: {}", p2);

    let runtime = Instant::now() - start;
    println!("Completed in {} ms", runtime.as_nanos() as f64 / 1e6);
}
