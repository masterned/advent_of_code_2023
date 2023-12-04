use regex::Regex;
use std::{collections::HashSet, error::Error, fmt, fs, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PartNumber {
    pub number: usize,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
struct Gear {
    coord: (usize, usize),
    ratios: HashSet<PartNumber>,
}

#[derive(Clone, Debug)]
struct Schematic {
    cells: Vec<Vec<Option<char>>>,
    parts: Vec<PartNumber>,
    gears: Vec<Gear>,
}

impl Schematic {
    fn is_valid_part_number(&self, part: &PartNumber) -> bool {
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

    fn get_valid_part_numbers(&self) -> Vec<&PartNumber> {
        self.parts
            .iter()
            .filter(|part| self.is_valid_part_number(part))
            .collect()
    }

    fn get_full_part_number(&self, x: usize, y: usize) -> Option<&PartNumber> {
        self.parts
            .iter()
            .filter(|part| part.line == y)
            .filter(|part| part.start <= x && part.end > x)
            .collect::<Vec<&PartNumber>>()
            .first()
            .copied()
    }

    fn populate_gear_ratios(&mut self) {
        let updated_gears = self
            .gears
            .iter()
            .map(|gear| {
                let (x, y) = gear.coord;

                let mut ratios = HashSet::new();

                [
                    (0, -1),  // North
                    (1, -1),  // North-East
                    (1, 0),   // East
                    (1, 1),   // South-East
                    (0, 1),   // South
                    (-1, 1),  // South-West
                    (-1, 0),  // West
                    (-1, -1), // North-West
                ]
                .iter()
                .for_each(|(x_d, y_d)| {
                    if let Some(x_n) = x.checked_add_signed(*x_d) {
                        if let Some(y_n) = y.checked_add_signed(*y_d) {
                            if let Some(full_number) = self.get_full_part_number(x_n, y_n) {
                                ratios.insert(full_number);
                            }
                        }
                    }
                });

                let ratios = ratios.iter().copied().cloned().collect();

                Gear {
                    ratios,
                    coord: (x, y),
                }
            })
            .collect();
        self.gears = updated_gears;
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
        let mut gears = Vec::new();

        let cells = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let re = Regex::new(r"\d+").unwrap();
                re.find_iter(line).for_each(|cap| {
                    let part = PartNumber {
                        number: cap.as_str().parse().unwrap(),
                        line: y,
                        start: cap.start(),
                        end: cap.end(),
                    };
                    parts.push(part);
                });

                let cells = line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => None,
                        '*' => {
                            gears.push(Gear {
                                coord: (x, y),
                                ratios: HashSet::new(),
                            });
                            Some('*')
                        }
                        c => Some(c),
                    })
                    .collect();

                cells
            })
            .collect();

        let mut schematic = Schematic {
            cells,
            parts,
            gears,
        };
        schematic.populate_gear_ratios();

        Ok(schematic)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day3")?;

    let scheme: Schematic = input.parse()?;

    let valid_part_number_sum: usize = scheme
        .get_valid_part_numbers()
        .iter()
        .map(|part| part.number)
        .sum();
    println!("Part 1: {valid_part_number_sum:?}");

    let valid_gears = scheme
        .gears
        .iter()
        .filter(|gear| gear.ratios.len() == 2)
        .map(|gear| gear.ratios.iter().map(|n| n.number).product::<usize>())
        .sum::<usize>();
    println!("Part 2: {valid_gears:?}");

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
                &PartNumber {
                    number: 467,
                    line: 0,
                    start: 0,
                    end: 3
                },
                &PartNumber {
                    number: 35,
                    line: 2,
                    start: 2,
                    end: 4
                },
                &PartNumber {
                    number: 633,
                    line: 2,
                    start: 6,
                    end: 9
                },
                &PartNumber {
                    number: 617,
                    line: 4,
                    start: 0,
                    end: 3,
                },
                &PartNumber {
                    number: 592,
                    line: 6,
                    start: 2,
                    end: 5,
                },
                &PartNumber {
                    number: 755,
                    line: 7,
                    start: 6,
                    end: 9,
                },
                &PartNumber {
                    number: 664,
                    line: 9,
                    start: 1,
                    end: 4,
                },
                &PartNumber {
                    number: 598,
                    line: 9,
                    start: 5,
                    end: 8
                }
            ],
            schematic.get_valid_part_numbers()
        );
    }
}
