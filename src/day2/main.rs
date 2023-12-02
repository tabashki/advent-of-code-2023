use std::env;
use std::fs;

#[derive(Debug)]
struct Hand {
    red: i32,
    green: i32,
    blue: i32,
}

impl Default for Hand {
    fn default() -> Self {
        Hand { red: 0, green: 0, blue: 0 }
    }
}

impl Hand {
    fn greater(&self, other: &Hand) -> bool {
        self.red > other.red ||
        self.green > other.green ||
        self.blue > other.blue
    }

    fn max(&mut self, other: &Hand) {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let mut total_power = 0;
    let mut valid_games = Vec::<i32>::new();

    let max_hand = Hand { red: 12, green: 13, blue: 14 };

    for line in input.lines() {
        let mut game_iter = line.split(": ");

        let game_header = game_iter.next().unwrap();
        let game_content = game_iter.next().unwrap();

        let game_id_str = game_header.strip_prefix("Game ").unwrap();
        let game_id = i32::from_str_radix(game_id_str, 10).unwrap();

        let mut game_valid = true;
        let mut required_hand = Hand { ..Default::default() };

        for cube_hand in game_content.split("; ") {
            let mut current_hand = Hand { ..Default::default() };

            for cube in cube_hand.split(", ") {
                let (value, color) = {
                    let mut iter = cube.split_whitespace();
                    let n = i32::from_str_radix(iter.next().unwrap(), 10).unwrap();
                    let c = iter.next().unwrap();
                    (n, c)
                };
                match color {
                    "red" => current_hand.red += value,
                    "green" => current_hand.green += value,
                    "blue" => current_hand.blue += value,
                    _ => unreachable!()
                };
            }

            if current_hand.greater(&max_hand) {
                game_valid = false;
            }

            required_hand.max(&current_hand);
        }
        
        if game_valid {
            valid_games.push(game_id);
        }

        let power = required_hand.power();
        println!("game {} -> required_hand: {:?}, power: {}", game_id, required_hand, power);

        total_power += power;
    }

    println!("valid games: {:?}", valid_games);
    println!("total power: {}", total_power);
}
