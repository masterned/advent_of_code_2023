use std::{error::Error, fmt, fs, str::FromStr};

#[derive(Clone, Debug, Default)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl Almanac {
    pub fn get_location(&self, seed_number: usize) -> usize {
        let soil = self
            .seed_to_soil
            .iter()
            .find_map(|m| m.apply(seed_number))
            .unwrap_or(seed_number);
        let fertilizer = self
            .soil_to_fertilizer
            .iter()
            .find_map(|m| m.apply(soil))
            .unwrap_or(soil);
        let water = self
            .fertilizer_to_water
            .iter()
            .find_map(|m| m.apply(fertilizer))
            .unwrap_or(fertilizer);
        let light = self
            .water_to_light
            .iter()
            .find_map(|m| m.apply(water))
            .unwrap_or(water);
        let tempurature = self
            .light_to_temperature
            .iter()
            .find_map(|m| m.apply(light))
            .unwrap_or(light);
        let humidity = self
            .temperature_to_humidity
            .iter()
            .find_map(|m| m.apply(tempurature))
            .unwrap_or(tempurature);
        self.humidity_to_location
            .iter()
            .find_map(|m| m.apply(humidity))
            .unwrap_or(humidity)
    }

    pub fn get_closest_seed_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|&seed_number| self.get_location(seed_number))
            .min()
            .unwrap_or(0)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ParseAlmanacError();

impl fmt::Display for ParseAlmanacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse Almanac")
    }
}

impl Error for ParseAlmanacError {}

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac = Almanac::default();

        let mut lines = s.lines();

        let seeds_line = lines.next().ok_or(ParseAlmanacError())?;

        let seeds: Vec<_> = seeds_line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        almanac.seeds = seeds;

        let _ = lines.nth(1);

        let mut seed_to_soil = lines.next().ok_or(ParseAlmanacError())?;
        while !seed_to_soil.is_empty() {
            let mapping: Mapping = seed_to_soil.parse().map_err(|_| ParseAlmanacError())?;
            almanac.seed_to_soil.push(mapping);
            seed_to_soil = lines.next().ok_or(ParseAlmanacError())?;
        }

        let _ = lines.next();

        let mut soil_to_fertilizer = lines.next().ok_or(ParseAlmanacError())?;
        while !soil_to_fertilizer.is_empty() {
            let mapping: Mapping = soil_to_fertilizer
                .parse()
                .map_err(|_| ParseAlmanacError())?;
            almanac.soil_to_fertilizer.push(mapping);
            soil_to_fertilizer = lines.next().ok_or(ParseAlmanacError())?;
        }

        let _ = lines.next();

        let mut fertilizer_to_water = lines.next().ok_or(ParseAlmanacError())?;
        while !fertilizer_to_water.is_empty() {
            let mapping: Mapping = fertilizer_to_water
                .parse()
                .map_err(|_| ParseAlmanacError())?;
            almanac.fertilizer_to_water.push(mapping);
            fertilizer_to_water = lines.next().ok_or(ParseAlmanacError())?;
        }

        let _ = lines.next();

        let mut water_to_light = lines.next().ok_or(ParseAlmanacError())?;
        while !water_to_light.is_empty() {
            let mapping: Mapping = water_to_light.parse().map_err(|_| ParseAlmanacError())?;
            almanac.water_to_light.push(mapping);
            water_to_light = lines.next().ok_or(ParseAlmanacError())?;
        }

        let _ = lines.next();

        let mut light_to_temperature = lines.next().ok_or(ParseAlmanacError())?;
        while !light_to_temperature.is_empty() {
            let mapping: Mapping = light_to_temperature
                .parse()
                .map_err(|_| ParseAlmanacError())?;
            almanac.light_to_temperature.push(mapping);
            light_to_temperature = lines.next().ok_or(ParseAlmanacError())?;
        }

        let _ = lines.next();

        let mut temperature_to_humidity = lines.next().ok_or(ParseAlmanacError())?;
        while !temperature_to_humidity.is_empty() {
            let mapping: Mapping = temperature_to_humidity
                .parse()
                .map_err(|_| ParseAlmanacError())?;
            almanac.temperature_to_humidity.push(mapping);
            temperature_to_humidity = lines.next().ok_or(ParseAlmanacError())?;
        }

        let _ = lines.next();

        let mut humidity_to_location = lines.next();
        while humidity_to_location.is_some() {
            let mapping: Mapping = humidity_to_location
                .ok_or(ParseAlmanacError())?
                .parse()
                .map_err(|_| ParseAlmanacError())?;
            almanac.humidity_to_location.push(mapping);
            humidity_to_location = lines.next();
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

#[derive(Clone, Copy, Debug, Default)]
struct ParseMappingError();

impl fmt::Display for ParseMappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse Mapping")
    }
}

impl Error for ParseMappingError {}

impl FromStr for Mapping {
    type Err = ParseMappingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s
            .split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();

        if split.len() != 3 {
            return Err(ParseMappingError());
        }

        let mut split = split.iter();

        let destination = *split.next().ok_or(ParseMappingError())?;
        let source = *split.next().ok_or(ParseMappingError())?;
        let range_length = *split.next().ok_or(ParseMappingError())?;

        Ok(Mapping {
            destination,
            source,
            range_length,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day5")?;

    let almanac: Almanac = input.parse()?;

    let closest_location = almanac.get_closest_seed_location();

    println!("Part 1: {closest_location}");

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
