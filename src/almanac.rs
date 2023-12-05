use std::str::FromStr;
use thiserror::Error;

pub struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil_map: AlmanacMap,
    soil_to_fertilizer_map: AlmanacMap,
    fertilizer_to_water_map: AlmanacMap,
    water_to_light_map: AlmanacMap,
    light_to_temperature_map: AlmanacMap,
    temperature_to_humidity_map: AlmanacMap,
    humidity_to_location_map: AlmanacMap,
}

#[derive(Default)]
pub struct AlmanacMap {
    values: Vec<RangeMap>,
}

pub struct RangeMap {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

#[derive(Debug, Error, PartialEq)]
pub enum AlmanacParseError {
    #[error("Invalid input: missing seeds")]
    MissingSeeds,
    #[error("Invalid input: invalid seed: {0}")]
    InvalidSeed(String),
    #[error("Invalid input: missing seed to soil map")]
    MissingSeedToSoilMap,
    #[error("Invalid input: missing soil to fertilizer map")]
    MissingSoilToFertilizerMap,
    #[error("Invalid input: missing fertilizer to water map")]
    MissingFertilizerToWaterMap,
    #[error("Invalid input: missing water to light map")]
    MissingWaterToLightMap,
    #[error("Invalid input: missing light to temperature map")]
    MissingLightToTemperatureMap,
    #[error("Invalid input: missing temperature to humidity map")]
    MissingTemperatureToHumidityMap,
    #[error("Invalid input: missing humidity to location map")]
    MissingHumidityToLocationMap,
    #[error("Invalid input: missing {0} map header")]
    MissingHeaderLine(String),
    #[error("Invalid input: invalid range: {0}")]
    InvalidRange(String),
    #[error("Invalid input: invalid value in range: {0}")]
    InvalidValueInRange(String),
}

impl FromStr for Almanac {
    type Err = AlmanacParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let seed_line = lines.next().ok_or(AlmanacParseError::MissingSeeds)?;
        if !seed_line.starts_with("seeds: ") {
            return Err(AlmanacParseError::MissingSeeds);
        }
        let seeds = seed_line[7..]
            .split_whitespace()
            .map(|n| {
                n.trim()
                    .parse::<usize>()
                    .map_err(|_| AlmanacParseError::InvalidSeed(n.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Consume the blank line
        let blank_line = lines
            .next()
            .ok_or(AlmanacParseError::MissingSeedToSoilMap)?;
        if !blank_line.is_empty() {
            return Err(AlmanacParseError::MissingSeedToSoilMap);
        }

        let seed_to_soil_map = AlmanacMap::from_lines(&mut lines, "seed-to-soil")?;
        let soil_to_fertilizer_map = AlmanacMap::from_lines(&mut lines, "soil-to-fertilizer")?;
        let fertilizer_to_water_map = AlmanacMap::from_lines(&mut lines, "fertilizer-to-water")?;
        let water_to_light_map = AlmanacMap::from_lines(&mut lines, "water-to-light")?;
        let light_to_temperature_map = AlmanacMap::from_lines(&mut lines, "light-to-temperature")?;
        let temperature_to_humidity_map =
            AlmanacMap::from_lines(&mut lines, "temperature-to-humidity")?;
        let humidity_to_location_map = AlmanacMap::from_lines(&mut lines, "humidity-to-location")?;

        Ok(Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        })
    }
}

impl Almanac {
    pub fn seed_to_soil(&self, seed: usize) -> usize {
        self.seed_to_soil_map.map(seed)
    }

    pub fn soil_to_fertilizer(&self, soil: usize) -> usize {
        self.soil_to_fertilizer_map.map(soil)
    }

    pub fn fertilizer_to_water(&self, fertilizer: usize) -> usize {
        self.fertilizer_to_water_map.map(fertilizer)
    }

    pub fn water_to_light(&self, water: usize) -> usize {
        self.water_to_light_map.map(water)
    }

    pub fn light_to_temperature(&self, light: usize) -> usize {
        self.light_to_temperature_map.map(light)
    }

    pub fn temperature_to_humidity(&self, temperature: usize) -> usize {
        self.temperature_to_humidity_map.map(temperature)
    }

    pub fn humidity_to_location(&self, humidity: usize) -> usize {
        self.humidity_to_location_map.map(humidity)
    }

    pub fn seed_to_location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil(seed);
        let fertilizer = self.soil_to_fertilizer(soil);
        let water = self.fertilizer_to_water(fertilizer);
        let light = self.water_to_light(water);
        let temperature = self.light_to_temperature(light);
        let humidity = self.temperature_to_humidity(temperature);
        self.humidity_to_location(humidity)
    }

    pub fn get_seed_locations(&self) -> Vec<usize> {
        self.seeds
            .iter()
            .map(|seed| self.seed_to_location(*seed))
            .collect()
    }
}

impl AlmanacMap {
    fn from_lines<'a>(
        lines: &mut impl Iterator<Item = &'a str>,
        map_prefix: &str,
    ) -> Result<Self, AlmanacParseError> {
        let header_line = lines
            .next()
            .ok_or_else(|| AlmanacParseError::MissingHeaderLine(map_prefix.to_string()))?;

        if header_line.trim() != format!("{} map:", map_prefix) {
            return Err(AlmanacParseError::MissingHeaderLine(map_prefix.to_string()));
        }

        let mut values = vec![];
        for line in lines {
            if line.is_empty() {
                break;
            }
            let range_map = line.parse::<RangeMap>()?;
            values.push(range_map);
        }
        Ok(AlmanacMap { values })
    }

    pub fn map(&self, value: usize) -> usize {
        for range_map in &self.values {
            if let Some(mapped_value) = range_map.map(value) {
                return mapped_value;
            }
        }
        value
    }
}

impl FromStr for RangeMap {
    type Err = AlmanacParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_whitespace()
            .map(|n| {
                n.trim()
                    .parse::<usize>()
                    .map_err(|_| AlmanacParseError::InvalidValueInRange(n.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if values.len() != 3 {
            return Err(AlmanacParseError::InvalidRange(s.to_string()));
        }

        let destination_start = values[0];
        let source_start = values[1];
        let range_length = values[2];
        Ok(RangeMap {
            destination_start,
            source_start,
            range_length,
        })
    }
}

impl RangeMap {
    pub fn map(&self, value: usize) -> Option<usize> {
        if value < self.source_start {
            return None;
        }
        if value >= self.source_start + self.range_length {
            return None;
        }
        Some(self.destination_start + (value - self.source_start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::Source;

    fn get_example_almanac() -> Almanac {
        let input = Source::try_from("inputs/day-5-example.txt").unwrap();
        let input = input.read_string().unwrap();
        Almanac::from_str(&input).unwrap()
    }

    #[test]
    fn test_parse_almanac() {
        let result = get_example_almanac();
        assert_eq!(result.seeds, vec![79, 14, 55, 13]);
        assert_eq!(result.seed_to_soil_map.values.len(), 2);
        assert_eq!(result.soil_to_fertilizer_map.values.len(), 3);
        assert_eq!(result.fertilizer_to_water_map.values.len(), 4);
        assert_eq!(result.water_to_light_map.values.len(), 2);
        assert_eq!(result.light_to_temperature_map.values.len(), 3);
        assert_eq!(result.temperature_to_humidity_map.values.len(), 2);
        assert_eq!(result.humidity_to_location_map.values.len(), 2);
    }

    #[test]
    fn test_parse_almanac_regular_input() {
        let input = Source::try_from("inputs/day-5-example.txt").unwrap();
        let input = input.read_string().unwrap();
        let result = Almanac::from_str(&input).unwrap();
        assert!(result.seeds.len() > 0);
    }

    #[test]
    fn test_parse_range_map() {
        let input = "1 2 3";
        let result = input.parse::<RangeMap>().unwrap();
        assert_eq!(result.destination_start, 1);
        assert_eq!(result.source_start, 2);
        assert_eq!(result.range_length, 3);
    }

    #[test]
    fn test_example_almanac_seed_to_soil_mapping() {
        let almanac = get_example_almanac();
        assert_eq!(almanac.seed_to_soil(79), 81);
        assert_eq!(almanac.seed_to_soil(14), 14);
        assert_eq!(almanac.seed_to_soil(55), 57);
        assert_eq!(almanac.seed_to_soil(13), 13);
    }

    #[test]
    fn test_example_almanac_lowest_location() {
        let almanac = get_example_almanac();
        let lowest_location = almanac.get_seed_locations().into_iter().min();
        assert_eq!(lowest_location, Some(35));
    }
}
