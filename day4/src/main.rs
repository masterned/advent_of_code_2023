use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day4")?;

    let score_total: usize = input
        .lines()
        .map(|card| {
            card.split(':').last().map_or(0, |content: &str| {
                let mut split = content.split('|');
                split.next().map_or(0, |winners_str: &str| {
                    split.last().map_or(0, |my_numbers_str: &str| {
                        let win_numbers: Vec<_> = winners_str.split_whitespace().collect();
                        let my_numbers = my_numbers_str.split_whitespace();

                        my_numbers.filter(|n| win_numbers.contains(n)).count()
                    })
                })
            })
        })
        .map(|count| {
            if count > 0 {
                2_usize.pow((count - 1) as u32)
            } else {
                0
            }
        })
        .sum();

    println!("Part 1: {score_total:?}");

    Ok(())
}
