use std::{error::Error, fmt, fs, str::FromStr};

use regex::Regex;

#[derive(Clone, Copy, Debug, Default)]
struct Game {
    pub id: usize,
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Game {
    pub fn could_contain(&self, other: &Game) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

struct ParseGameError();

impl fmt::Display for ParseGameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse Game.")
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game_content_separation = s.split(':');
        let game_details = game_content_separation.next().ok_or(ParseGameError())?;

        let game_id = game_details
            .split_whitespace()
            .skip(1)
            .next()
            .ok_or(ParseGameError())?
            .parse::<usize>()
            .map_err(|_| ParseGameError())?;

        let game_rounds = game_content_separation
            .next()
            .ok_or(ParseGameError())?
            .split(';')
            .fold(Game::default(), |mut acc, next| {
                let cubes = next.split(',');

                cubes.for_each(|handful| {
                    let re = Regex::new("(\\d+)\\s*(red|blue|green)").unwrap();
                    if let Some(caps) = re.captures(handful) {
                        match &caps[2] {
                            "red" => acc.red = acc.red.max(caps[1].parse::<usize>().unwrap()),
                            "green" => acc.green = acc.green.max(caps[1].parse::<usize>().unwrap()),
                            "blue" => acc.blue = acc.blue.max(caps[1].parse::<usize>().unwrap()),
                            _ => unreachable!(),
                        }
                    }
                });

                acc
            });

        let game = Game {
            id: game_id,
            ..game_rounds
        };
        // println!("{game:?}");

        Ok(game)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day2")?;
    let lines = input.lines();

    let bag = Game {
        id: 0,
        red: 12,
        green: 13,
        blue: 14,
    };

    let games: Vec<_> = lines.filter_map(|line| line.parse::<Game>().ok()).collect();

    let valid_game_id_sum: usize = games
        .iter()
        .filter(|game| bag.could_contain(game))
        .map(|game| game.id)
        .sum();

    println!("Part 1: {valid_game_id_sum}");

    let game_powers_sum: usize = games
        .iter()
        .map(|game| game.red * game.green * game.blue)
        .sum();
    println!("Part 2: {game_powers_sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _should_return_false_on_empty_bag() {
        let bag = Game::default();
        assert!(!bag.could_contain(&Game {
            red: 4,
            green: 2,
            blue: 6,
            ..Game::default()
        }));
    }
}
