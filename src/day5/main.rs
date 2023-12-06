extern crate rayon;
use std::cmp::Ordering;
use std::str::Lines;
use std::env;
use std::fs;
use std::vec;
use rayon::prelude::*;


#[derive(Debug, Copy, Clone)]
struct RangeMap {
    dst: usize,
    src: usize,
    len: usize,
}

#[derive(Clone)]
struct MultiRangeMap {
    ranges: Vec<RangeMap>
}

struct Almanac {
    seeds: Vec<usize>,
    seed_mapping_stack: Vec<MultiRangeMap>,
}

// -------------------------------------------------------------------------- //

impl RangeMap {
    fn in_src_range(&self, index: usize) -> bool {
        index >= self.src && index < (self.src + self.len)
    }

    fn map(&self, index: usize) -> Option<usize> {
        if self.in_src_range(index) {
            let offset = index - self.src;
            Some(self.dst + offset)
        } else {
            None
        }
    }
}

impl MultiRangeMap {
    fn new(ranges: &[RangeMap]) -> MultiRangeMap {
        let mut srm = MultiRangeMap { ranges: ranges.into() };
        srm.sort_by_src();
        srm
    }

    fn sort_by_src(&mut self) {
        self.ranges.sort_by(|a, b| {
            a.src.cmp(&b.src)
        });
    }

    fn find_pivot(&self, index: usize) -> Result<usize, usize> {
        let pivot = self.ranges.binary_search_by(|r| {
            if r.in_src_range(index) {
                Ordering::Equal
            } else {
                r.src.cmp(&index)
            }
        });

        pivot
    }

    fn map(&self, index: usize) -> Option<usize> {
        let p = self.find_pivot(index);
        match p {
            Ok(i) => self.ranges[i].map(index),
            _ => None,
        }
    }
}

impl Almanac {
    fn seed_to_location(&self, seed: usize) -> usize {
        let mut index = seed;
        for m in &self.seed_mapping_stack {
            index = match m.map(index) {
                Some(i) => i,
                None => index,
            };
        }
        index
    }
}

// -------------------------------------------------------------------------- //

fn part1(almanac: &Almanac) -> usize {
    let mut minimum: usize = usize::MAX;
    
    for seed in &almanac.seeds {
        let location = almanac.seed_to_location(*seed);
        minimum = minimum.min(location);
    }

    minimum
}

fn part2(almanac: &Almanac) -> usize {
    let ranges: Vec<(usize, usize)> = almanac.seeds.chunks(2).map(|c| (c[0], c[1])).collect();
    let mut minimum = usize::MAX;

    // Really dumb, parallel brute-force solution
    for (start, len) in ranges {
        print!("seeds: [{}, {}) -> ", start, start + len);
        
        let iter = (start..(start + len)).into_par_iter();
        let min_loc = iter.map(|seed| almanac.seed_to_location(seed)).min().unwrap();
        println!("min loc: {}", min_loc);
        
        minimum = minimum.min(min_loc);
    }

    minimum
}

fn parse_mappings(lines: &mut Lines<'_>, expected_header: &str) -> MultiRangeMap {
    loop {
        // Can't use skip_while since that moves away lines
        let header = lines.next().unwrap();
        if !header.is_empty() {
            assert_eq!(header, expected_header);
            break;
        }
    }

    let mut ranges: Vec<RangeMap> = Vec::new();
    for line in lines.take_while(|l| !l.is_empty()) {
        let nums: Vec<usize> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        assert_eq!(nums.len(), 3);

        ranges.push(RangeMap {
            dst: nums[0],
            src: nums[1],
            len: nums[2], 
        });
    }

    MultiRangeMap::new(&ranges)
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let mut lines = input.lines();

    let seeds_to_plant = {
        let header = lines.next().unwrap();
        let seed_split = header.split(": ").nth(1).unwrap();
        
        let mut seed_vec = vec![0usize; 0];
        for seed in seed_split.split(" ") {
            seed_vec.push(seed.parse().unwrap());
        }
        seed_vec
    };

    let mut mappings: Vec<MultiRangeMap> = Vec::new();

    mappings.push(parse_mappings(&mut lines, "seed-to-soil map:"));
    mappings.push(parse_mappings(&mut lines, "soil-to-fertilizer map:"));
    mappings.push(parse_mappings(&mut lines, "fertilizer-to-water map:"));
    mappings.push(parse_mappings(&mut lines, "water-to-light map:"));
    mappings.push(parse_mappings(&mut lines, "light-to-temperature map:"));
    mappings.push(parse_mappings(&mut lines, "temperature-to-humidity map:"));
    mappings.push(parse_mappings(&mut lines, "humidity-to-location map:"));

    let almanac = Almanac {
        seeds: seeds_to_plant,
        seed_mapping_stack: mappings,
    };

    let p1 = part1(&almanac);
    let p2 = part2(&almanac);

    println!("part 1 result = {}", p1);
    println!("part 2 result = {}", p2);
}
