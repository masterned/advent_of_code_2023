#![warn(clippy::pedantic)]

use std::{error::Error, fmt, fs, ops::Range, str::FromStr};

#[derive(Clone, Debug, Default)]
pub struct Almanac {
    seeds: Vec<usize>,
    seed_ranges: Vec<Range<usize>>,
    rules: Vec<Rule>,
}

impl Almanac {
    pub fn get_location(&self, seed_number: usize) -> usize {
        let mut result = seed_number;

        for rule in &self.rules {
            result = rule.apply_to_number(result);
        }

        result
    }

    pub fn get_closest_seed_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|&seed_number| self.get_location(seed_number))
            .min()
            .unwrap_or(0)
    }

    pub fn get_closest_location_of_seed_ranges(&self) -> usize {
        self.seed_ranges
            .iter()
            .map(|seed_range| {
                let mut min = self.get_location(seed_range.start);
                for seed_number in seed_range.start..seed_range.end {
                    min = min.min(self.get_location(seed_number));
                }
                min
            })
            .min()
            .unwrap_or(0)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum ParseAlmanacError {
    #[default]
    MissingField,
    RuleError,
}

impl fmt::Display for ParseAlmanacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unable to parse Almanac: {}",
            match self {
                ParseAlmanacError::MissingField => "missing field",
                ParseAlmanacError::RuleError => "unable to parse rule",
            }
        )
    }
}

impl Error for ParseAlmanacError {}

impl From<ParseRuleError> for ParseAlmanacError {
    fn from(_value: ParseRuleError) -> Self {
        Self::RuleError
    }
}

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac = Almanac::default();

        let mut sections = s.split("\n\n");

        let seeds_line = sections.next().ok_or(ParseAlmanacError::default())?;

        let seeds: Vec<_> = seeds_line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        let seed_ranges = seeds.clone();
        almanac.seeds = seeds;

        let seed_ranges: Vec<Range<usize>> = seed_ranges
            .chunks_exact(2)
            .map(|a| a[0]..a[0] + a[1])
            .collect();
        almanac.seed_ranges = seed_ranges;

        for section in sections {
            almanac.rules.push(section.parse()?);
        }

        Ok(almanac)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Mapping {
    destination: usize,
    source: usize,
    range_length: usize,
}

impl Mapping {
    pub fn apply(&self, value: usize) -> Option<usize> {
        if (self.source..(self.source + self.range_length)).contains(&value) {
            return Some(value - self.source + self.destination);
        }
        None
    }
}

impl FromStr for Mapping {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s
            .split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();

        if split.len() != 3 {
            return Err(ParseRuleError());
        }

        let mut split = split.iter();

        let destination = *split.next().ok_or(ParseRuleError())?;
        let source = *split.next().ok_or(ParseRuleError())?;
        let range_length = *split.next().ok_or(ParseRuleError())?;

        Ok(Mapping {
            destination,
            source,
            range_length,
        })
    }
}

#[derive(Clone, Debug, Default)]
struct Rule(Vec<Mapping>);

impl Rule {
    pub fn apply_to_number(&self, number: usize) -> usize {
        self.0
            .iter()
            .find_map(|m| m.apply(number))
            .unwrap_or(number)
    }
}

impl From<Vec<Mapping>> for Rule {
    fn from(value: Vec<Mapping>) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ParseRuleError();

impl fmt::Display for ParseRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse Rule")
    }
}

impl Error for ParseRuleError {}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rule = vec![];
        let mut lines = s.lines();

        let _title = lines.next();

        for line in lines {
            rule.push(line.parse()?);
        }

        Ok(Self(rule))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/ex5")?;

    let almanac: Almanac = input.parse()?;
    // println!("{almanac:?}");

    let closest_location = almanac.get_closest_seed_location();
    println!("Part 1: {closest_location}");

    let closest_range_location = almanac.get_closest_location_of_seed_ranges();
    println!("Part 2: {closest_range_location}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod almanac {}

    mod mapping {
        use super::*;

        #[test]
        fn _should_parse_mapping() {
            let mapping: Mapping = "50 98 2".parse().expect("Should be able to parse this");

            assert_eq!(
                Mapping {
                    destination: 50,
                    source: 98,
                    range_length: 2
                },
                mapping
            );
        }

        #[test]
        fn _should_keep_relative_position_on_mapping_apply() {
            let mapping = Mapping {
                destination: 50,
                source: 98,
                range_length: 2,
            };

            assert_eq!(mapping.apply(98), Some(50));
            assert_eq!(mapping.apply(99), Some(51));
        }
    }
}
