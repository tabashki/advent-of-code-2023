use std::env;
use std::fs;


#[derive(Debug)]
struct Race {
    time_allowed: usize,
    distance_record: usize,
}

// -------------------------------------------------------------------------- //

fn count_winning_permutations(races: &[Race]) -> usize {
    let mut result = 1;

    for r in races {
        let mut wins = 0;

        for hold_time in 1..r.time_allowed {
            let travel_time = r.time_allowed - hold_time; 
            let speed = hold_time;
            let dist = speed * travel_time;

            if dist > r.distance_record {
                wins += 1;
            }
        }

        println!("{:?} -> wins: {}", r, wins);
        result *= wins;
    }

    result
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    // Part 1 input
    let mut lines = input.lines();
    let times_str = lines.next().unwrap().strip_prefix("Time:").unwrap().trim();
    let dist_str = lines.next().unwrap().strip_prefix("Distance:").unwrap().trim();

    let times: Vec<usize> = {
        times_str.split(" ").filter_map(|t| t.parse().ok()).collect()
    };
    let distances: Vec<usize> = {
        dist_str.split(" ").filter_map(|d| d.parse().ok()).collect()
    };

    assert_eq!(times.len(), distances.len());
    let races: Vec<Race> = times.iter().zip(distances.iter()).map(|(t, d)|
        Race { time_allowed: *t, distance_record: *d }
    ).collect();

    // Part 2 input
    let combined_time: usize = {
        times_str.split(" ").filter(|s| !s.is_empty())
            .fold(String::new(), |a, b| a + b).parse().unwrap()
    };

    let combined_distance: usize = {
        dist_str.split(" ").filter(|s| !s.is_empty())
            .fold(String::new(), |a, b| a + b).parse().unwrap()
    };

    let combined_race = Race {
        time_allowed: combined_time,
        distance_record: combined_distance,
    };

    let p1 = count_winning_permutations(&races);
    println!("Part 1 result: {}", p1);
    
    let p2 = count_winning_permutations(&[combined_race]);
    println!("Part 2 result = {}", p2);
}
