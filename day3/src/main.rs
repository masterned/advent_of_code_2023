use regex::Regex;
use std::{error::Error, fmt, fs, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part {
    pub number: usize,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
struct Schematic {
    cells: Vec<Vec<Option<char>>>,
    parts: Vec<Part>,
}

impl Schematic {
    fn is_valid_part(&self, part: &Part) -> bool {
        let mut range_start = part.start;
        if part.start > 0 {
            range_start = part.start - 1;

            if self
                .cells
                .get(part.line)
                .is_some_and(|r| r.get(part.start - 1).is_some_and(|c| c.is_some()))
            {
                return true;
            }
        }
        for x in range_start..(part.end + 1) {
            if part.line > 0
                && self
                    .cells
                    .get(part.line - 1)
                    .is_some_and(|r| r.get(x).is_some_and(|c| c.is_some()))
            {
                return true;
            }
            if self
                .cells
                .get(part.line + 1)
                .is_some_and(|r| r.get(x).is_some_and(|c| c.is_some()))
            {
                return true;
            }
        }
        if self
            .cells
            .get(part.line)
            .is_some_and(|r| r.get(part.end).is_some_and(|c| c.is_some()))
        {
            return true;
        }
        false
    }

    fn get_valid_parts(&self) -> Vec<&Part> {
        self.parts
            .iter()
            .filter(|part| self.is_valid_part(part))
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
struct ParseSchematicError();

impl fmt::Display for ParseSchematicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse Schematic")
    }
}

impl Error for ParseSchematicError {}

impl FromStr for Schematic {
    type Err = ParseSchematicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = Vec::new();

        let cells = s
            .lines()
            .enumerate()
            .map(|(i, l)| {
                let re = Regex::new(r"\d+").unwrap();
                re.find_iter(l).for_each(|cap| {
                    let part = Part {
                        number: cap.as_str().parse().unwrap(),
                        line: i,
                        start: cap.start(),
                        end: cap.end(),
                    };
                    parts.push(part);
                });

                let mut line_width = 0;

                let cells = l
                    .chars()
                    .map(|c| {
                        line_width += 1;
                        match c {
                            '.' => None,
                            x => Some(x),
                        }
                    })
                    .collect();

                cells
            })
            .collect();
        Ok(Schematic { cells, parts })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day3")?;

    let scheme: Schematic = input.parse()?;

    let valid_part_number_sum: usize = scheme
        .get_valid_parts()
        .iter()
        .map(|part| part.number)
        .sum();
    println!("Part 1: {valid_part_number_sum:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn should_get_valid_part_numbers() {
        let schematic: Schematic = EXAMPLE_INPUT.parse().unwrap();
        assert_eq!(
            vec![
                &Part {
                    number: 467,
                    line: 0,
                    start: 0,
                    end: 3
                },
                &Part {
                    number: 35,
                    line: 2,
                    start: 2,
                    end: 4
                },
                &Part {
                    number: 633,
                    line: 2,
                    start: 6,
                    end: 9
                },
                &Part {
                    number: 617,
                    line: 4,
                    start: 0,
                    end: 3,
                },
                &Part {
                    number: 592,
                    line: 6,
                    start: 2,
                    end: 5,
                },
                &Part {
                    number: 755,
                    line: 7,
                    start: 6,
                    end: 9,
                },
                &Part {
                    number: 664,
                    line: 9,
                    start: 1,
                    end: 4,
                },
                &Part {
                    number: 598,
                    line: 9,
                    start: 5,
                    end: 8
                }
            ],
            schematic.get_valid_parts()
        );
    }
}
