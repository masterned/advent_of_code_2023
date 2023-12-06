#![warn(clippy::pedantic)]

use std::{error::Error, fmt, fs, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Card {
    pub count: usize,
    pub win_count: usize,
}

impl Card {
    pub fn calculate_score(count: usize) -> usize {
        if count > 0 {
            1 << (count - 1)
        } else {
            0
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            count: 1,
            win_count: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ParseCardError();

impl fmt::Display for ParseCardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse card")
    }
}

impl Error for ParseCardError {}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':').last().ok_or(ParseCardError())?.split('|');

        let winning_numbers: Vec<_> = split
            .next()
            .ok_or(ParseCardError())?
            .split_whitespace()
            .collect();

        let win_count = split
            .next()
            .ok_or(ParseCardError())?
            .split_whitespace()
            .filter(|n| winning_numbers.contains(n))
            .count();

        Ok(Card {
            win_count,
            ..Default::default()
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day4")?;

    let mut cards: Vec<_> = input
        .lines()
        .filter_map(|line| line.parse::<Card>().ok())
        .collect();

    let win_counts: Vec<_> = cards.iter().map(|card| card.win_count).collect();

    let score_total: usize = win_counts
        .iter()
        .map(|&count| Card::calculate_score(count))
        .sum();
    println!("Part 1: {score_total:?}");

    let mut multiplier = 1;
    for (i, win_count) in win_counts.iter().enumerate() {
        for n in 0..*win_count {
            if let Some(card) = cards.get_mut(i + n + 1) {
                card.count += multiplier;
            }
        }
        if let Some(card) = cards.get(i) {
            multiplier += card.count;
        }
    }
    let card_count = cards.iter().map(|card| card.count).sum::<usize>();
    println!("Part 2: {card_count}");

    Ok(())
}
