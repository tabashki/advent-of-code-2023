use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::env;
use std::fs;
use std::str::Lines;


#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Clone, Copy)]
struct CardHand {
    cards: [u16; 5],
    hand_type: HandType,
    bid: usize,
}

// -------------------------------------------------------------------------- //

impl fmt::Display for CardHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_to_char = |&n| {
            match n {
                1 => '*',
                2..=9 => ((n + 0x30) as u8) as char,
                10 => 'T',
                11 => 'J',
                12 => 'Q',
                13 => 'K',
                14 => 'A',
                _ => unreachable!(),
            }
        };
        let c = String::from_iter(self.cards.iter().map(num_to_char));
        write!(f, "CardHand {{ '{}', {:?}, bid: {} }}", c, self.hand_type, self.bid)
    }
}

impl CardHand {
    fn new(hand: &str, bid: usize, use_wildcards: bool) -> CardHand {
        let chars: Vec<char> = hand.chars().collect();
        assert_eq!(chars.len(), 5);

        let char_to_num = |&c| {
            match c {
                'J' => if use_wildcards { 1 } else { 11 },
                '2'..='9' => (c as u16) - 0x30,
                'T' => 10,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            }
        };
        let nums: Vec<u16> = chars.iter().map(char_to_num).collect();
        let mut occurences: HashMap<u16, u16> = HashMap::new();
        for n in &nums {
            match occurences.get_mut(n) {
                Some(occur) => { *occur += 1; },
                None => { occurences.insert(*n, 1); },
            };
        }
        let wildcards = match occurences.get(&1) {
            Some(o) => *o,
            None => 0,
        };
        occurences.remove(&1);

        let max_occur = match occurences.values().max() {
            Some(m) => *m,
            None => 0,
        } + wildcards;

        let ht = match occurences.len() {
            0 => {
                assert_eq!(wildcards, 5);
                HandType::FiveKind
            }
            1 => HandType::FiveKind,
            2 => if max_occur == 4 {
                    HandType::FourKind
                } else {
                    HandType::FullHouse
                },
            3 => if max_occur == 3 {
                    HandType::ThreeKind
                } else {
                    HandType::TwoPair
                },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        };

        CardHand {
            cards: nums.try_into().unwrap(),
            hand_type: ht,
            bid: bid,
        }
    } 

    fn cmp_strength(&self, other: &CardHand) -> Ordering {
        let s = self.hand_type.partial_cmp(&other.hand_type).unwrap();
        match s {
            Ordering::Equal => self.cards.cmp(&other.cards),
            _ => s,
        }
    }
}

fn parse_and_sort_card_hands(lines: &Lines<'_>, use_wildcard: bool) -> Vec<CardHand> {
    let mut hands: Vec<CardHand> = Vec::new();

    for line in lines.clone() {
        let mut iter = line.split(" ");
        let hand = iter.next().unwrap();
        let bid = iter.next().unwrap().parse().unwrap();
        hands.push(CardHand::new(hand, bid, use_wildcard));
    }

    hands.sort_by(|a: &CardHand, b| {
        a.cmp_strength(b)
    });

    hands
}

fn total_winning(hands: &[CardHand]) -> usize {
    let mut total = 0;
    for i in 0..hands.len() {
        let rank = i + 1;
        let bid = hands[i].bid as usize;
        let win = rank * bid;

        println!("[rank {}] {}", rank, hands[i]);
        total += win;
    }
    total
}

fn part1(lines: &Lines<'_>) -> usize {
    let use_wildcard = false;
    let hands = parse_and_sort_card_hands(lines, use_wildcard);
    total_winning(&hands)
}

fn part2(lines: &Lines<'_>) -> usize {
    let use_wildcard = true;
    let hands = parse_and_sort_card_hands(lines, use_wildcard);
    total_winning(&hands)
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let lines = input.lines();

    let p1 = part1(&lines);
    println!("Part 1 result: {}", p1);

    let p2 = part2(&lines);
    println!("Part 2 result: {}", p2);
}
