use std::env;
use std::fs;
use std::time::Instant;

// -------------------------------------------------------------------------- //

fn derivatives(values: &[i64]) -> Vec<i64> {
    assert!(values.len() > 1);

    let derivs: Vec<i64> = values.windows(2).map(|w| {
        match w {
            &[a, b] => b - a,
            _ => unreachable!(),
        }
    }).collect();

    derivs
}

fn calculate_derivatives(values: &[i64]) -> Vec<Vec<i64>> {
    let mut nth_deriv: Vec<Vec<i64>> = Vec::new();
    nth_deriv.push(Vec::from(values));

    loop {
        let mut mini = i64::MAX;
        let mut maxi = i64::MIN;
        let last_deriv = nth_deriv.last().unwrap();

        last_deriv.iter().for_each(|d| {
            mini = mini.min(*d);
            maxi = maxi.max(*d);
        });
        
        if mini == maxi {
            nth_deriv.push(Vec::from_iter([0]));
            break;
        }

        nth_deriv.push(derivatives(&last_deriv));
    }

    nth_deriv
}

fn part1(histories: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;

    for history in histories {
        let mut nth_deriv = calculate_derivatives(&history);

        for fwd_i in 1..nth_deriv.len() {
            let i = nth_deriv.len() - fwd_i - 1;

            let higher_deriv = *nth_deriv[i + 1].last().unwrap();
            let curr_deriv = nth_deriv[i].last().unwrap();

            let extrap = curr_deriv + higher_deriv;
            nth_deriv[i].push(extrap);  
        }

        let extrapolated = *nth_deriv[0].last().unwrap();
        sum += extrapolated;
    }

    sum
}

fn part2(histories: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;

    for history in histories {
        let mut nth_deriv = calculate_derivatives(&history);

        for fwd_i in 1..nth_deriv.len() {
            let i = nth_deriv.len() - fwd_i - 1;

            let higher_deriv = *nth_deriv[i + 1].first().unwrap();
            let curr_deriv = nth_deriv[i].first().unwrap();

            let extrap = curr_deriv - higher_deriv;
            nth_deriv[i].insert(0, extrap);
        }

        let extrapolated = *nth_deriv[0].first().unwrap();
        sum += extrapolated;
    }

    sum
}

fn main() {
    let start = Instant::now();

    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let lines = input.lines();
    let mut histories: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        let hist: Vec<i64> = line.split(" ").map(|v| v.trim().parse().unwrap()).collect();
        histories.push(hist);
    }

    let p1 = part1(&histories);
    println!("Part 1 result: {}", p1);
    
    let p2 = part2(&histories);
    println!("Part 2 result = {}", p2);

    let runtime = Instant::now() - start;
    println!("Completed in {} ms", runtime.as_nanos() as f64 / 1e6);
}
