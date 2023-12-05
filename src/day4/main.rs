use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
    intersect_count: usize,
}

impl Card {
    fn new(id: u32, winning: Vec<u32>, have: Vec<u32>) -> Card {
        let win_set = HashSet::from_iter(winning);
        let have_set = HashSet::from_iter(have);
        let intersect = win_set.intersection(&have_set).count();

        Card {
            id: id,
            winning: win_set,
            have: have_set,
            intersect_count: intersect,
        }
    }
}

// -------------------------------------------------------------------------- //

fn part1(cards: &[Card]) -> usize {
    let mut sum = 0;

    for card in cards {
        let win_count = card.intersect_count;
        if win_count > 0 {
            let points = 1 << (win_count - 1);
            sum += points;
        }
    }
    sum
}

fn depth_first_count_copies(cards: &[Card], depth: usize) -> usize {
    let mut count = 1;

    let card = &cards[0];
    let win_count = card.intersect_count;

    for i in 1..win_count+1 {
        count += depth_first_count_copies(&cards[i..], depth + 1);
    }

    count
}

fn part2(cards: &[Card]) -> usize {
    let mut sum = 0;

    for i in 0..cards.len() {
        sum += depth_first_count_copies(&cards[i..], 0);
    }

    sum
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let mut head_iter = line.split(": ");
        let header = head_iter.next().unwrap();
        let body = head_iter.next().unwrap();

        let id_str = header.strip_prefix("Card ").unwrap().trim();
        let id = u32::from_str_radix(id_str, 10).unwrap();

        let mut body_iter = body.split(" | ");
        let winning_str = body_iter.next().unwrap()
            .split(" ").filter_map(|s| if s.is_empty() { None } else { Some(s.trim()) });
        let have_str = body_iter.next().unwrap()
            .split(" ").filter_map(|s| if s.is_empty() { None } else { Some(s.trim()) });

        let mut win: Vec<u32> = Vec::new();
        let mut have: Vec<u32> = Vec::new();

        for w in winning_str {
            let n = w.parse().unwrap();
            win.push(n);
        }
        for h in have_str {
            let n = h.parse().unwrap();
            have.push(n);
        }

        let card = Card::new(id, win, have);
        cards.push(card);
    }

    let p1 = part1(&cards);
    let p2 = part2(&cards);

    println!("part 1 result = {}", p1);
    println!("part 2 result = {}", p2);
}
