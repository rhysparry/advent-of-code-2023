use log::{debug, trace};
use std::cmp::min;
use std::ops::Range;
use std::str::FromStr;
use thiserror::Error;

pub struct Almanac {
    seeds: Vec<usize>,
    seed_ranges: Vec<Range<usize>>,
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
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
    #[error("Invalid input: insufficient seed numbers for seed range")]
    InsufficientSeedNumbers,
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

        let seed_ranges = get_seed_ranges(&seeds)?;

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
            seed_ranges,
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

    pub fn seed_to_soil_ranges(&self, seed: &Range<usize>) -> Vec<Range<usize>> {
        self.seed_to_soil_map.map_ranges(seed)
    }

    pub fn soil_to_fertilizer_ranges(&self, soil: &Range<usize>) -> Vec<Range<usize>> {
        self.soil_to_fertilizer_map.map_ranges(soil)
    }

    pub fn fertilizer_to_water_ranges(&self, fertilizer: &Range<usize>) -> Vec<Range<usize>> {
        self.fertilizer_to_water_map.map_ranges(fertilizer)
    }

    pub fn water_to_light_ranges(&self, water: &Range<usize>) -> Vec<Range<usize>> {
        self.water_to_light_map.map_ranges(water)
    }

    pub fn light_to_temperature_ranges(&self, light: &Range<usize>) -> Vec<Range<usize>> {
        self.light_to_temperature_map.map_ranges(light)
    }

    pub fn temperature_to_humidity_ranges(&self, temperature: &Range<usize>) -> Vec<Range<usize>> {
        self.temperature_to_humidity_map.map_ranges(temperature)
    }

    pub fn humidity_to_location_ranges(&self, humidity: &Range<usize>) -> Vec<Range<usize>> {
        self.humidity_to_location_map.map_ranges(humidity)
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

    pub fn seed_range_to_location_ranges(&self, seed_range: &Range<usize>) -> Vec<Range<usize>> {
        debug!("seed range: {:?}", seed_range);
        let soil_ranges = self.seed_to_soil_ranges(seed_range);
        debug!("soil ranges: {:?}", soil_ranges);
        let fertilizer_ranges = soil_ranges
            .iter()
            .flat_map(|soil_range| self.soil_to_fertilizer_ranges(soil_range))
            .collect::<Vec<_>>();
        debug!("fertilizer ranges: {:?}", fertilizer_ranges);
        let water_ranges = fertilizer_ranges
            .iter()
            .flat_map(|fertilizer_range| self.fertilizer_to_water_ranges(fertilizer_range))
            .collect::<Vec<_>>();
        debug!("water ranges: {:?}", water_ranges);
        let light_ranges = water_ranges
            .iter()
            .flat_map(|water_range| self.water_to_light_ranges(water_range))
            .collect::<Vec<_>>();
        debug!("light ranges: {:?}", light_ranges);
        let temperature_ranges = light_ranges
            .iter()
            .flat_map(|light_range| self.light_to_temperature_ranges(light_range))
            .collect::<Vec<_>>();
        debug!("temperature ranges: {:?}", temperature_ranges);
        let humidity_ranges = temperature_ranges
            .iter()
            .flat_map(|temperature_range| self.temperature_to_humidity_ranges(temperature_range))
            .collect::<Vec<_>>();
        debug!("humidity ranges: {:?}", humidity_ranges);
        let location_ranges = humidity_ranges
            .iter()
            .flat_map(|humidity_range| self.humidity_to_location_ranges(humidity_range))
            .collect::<Vec<_>>();
        debug!("location ranges: {:?}", location_ranges);
        location_ranges
    }

    pub fn get_seed_locations(&self) -> Vec<usize> {
        self.seeds
            .iter()
            .map(|seed| self.seed_to_location(*seed))
            .collect()
    }

    pub fn iter_all_seeds<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.seed_ranges.iter().flat_map(|range| range.clone())
    }

    pub fn iter_all_seed_locations<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.iter_all_seeds()
            .map(|seed| self.seed_to_location(seed))
    }

    pub fn get_seed_location_ranges(&self) -> Vec<Range<usize>> {
        self.seed_ranges
            .iter()
            .flat_map(|seed_range| self.seed_range_to_location_ranges(seed_range))
            .collect()
    }
}

impl AlmanacMap {
    fn new(mut values: Vec<RangeMap>) -> Self {
        values.sort_by_key(|range_map| range_map.source_start);
        AlmanacMap { values }
    }
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
        Ok(AlmanacMap::new(values))
    }

    pub fn map(&self, value: usize) -> usize {
        for range_map in &self.values {
            if let Some(mapped_value) = range_map.map(value) {
                return mapped_value;
            }
        }
        value
    }

    fn find_next_range_map(&self, value: usize) -> Option<&RangeMap> {
        self.values.iter().find(|range_map| {
            range_map.source_start >= value || range_map.range_in().contains(&value)
        })
    }

    pub fn map_ranges(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        trace!("map_ranges: {:?}", range);
        let mut result = Vec::new();

        let mut pos = range.start;

        while pos < range.end {
            if let Some(range_map) = self.find_next_range_map(pos) {
                let range_in = range_map.range_in();
                trace!("range_in: {:?}", range_in);
                let r = if range_in.contains(&pos) {
                    trace!("range_in contains {:?}", pos);
                    let end_pos = min(range_in.end, range.end);
                    let start_range = range_map.map(pos).unwrap();
                    let end_range = start_range + (end_pos - pos);
                    pos = end_pos;
                    start_range..end_range
                } else {
                    trace!("range_in does not contain {:?}", pos);
                    let start = pos;
                    pos = range_in.start;
                    start..range_in.start
                };
                result.push(r);
            } else {
                trace!("no range_map found for {:?}", pos);
                result.push(pos..range.end);
                break;
            }
        }

        result
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

    pub fn range_in(&self) -> Range<usize> {
        self.source_start..(self.source_start + self.range_length)
    }

    pub fn range_out(&self) -> Range<usize> {
        self.destination_start..(self.destination_start + self.range_length)
    }
}

fn get_seed_ranges(seeds: &[usize]) -> Result<Vec<Range<usize>>, AlmanacParseError> {
    if seeds.len() % 2 != 0 {
        return Err(AlmanacParseError::InsufficientSeedNumbers);
    }

    let seed_ranges = seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect();

    Ok(seed_ranges)
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

    #[test]
    fn test_example_almanac_all_seeds_lowest_location() {
        let almanac = get_example_almanac();
        let lowest_location = almanac.iter_all_seed_locations().min();
        assert_eq!(lowest_location, Some(46));
    }

    #[test]
    fn test_example_almanac_map_ranges() {
        let almanac = get_example_almanac();
        let ranges = almanac.seed_to_soil_map.map_ranges(&(96..103));
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0], 98..100);
        assert_eq!(ranges[1], 50..52);
        assert_eq!(ranges[2], 100..103);
    }

    #[test]
    fn test_example_almanac_seed_ranges_lowest_location() {
        let almanac = get_example_almanac();
        let lowest_location = almanac
            .seed_ranges
            .iter()
            .flat_map(|seed_range| almanac.seed_range_to_location_ranges(seed_range))
            .map(|location_range| location_range.start)
            .min();
        assert_eq!(lowest_location, Some(46));
    }

    #[test]
    fn test_example_soil_range() {
        let almanac = get_example_almanac();
        let soil_ranges = almanac.seed_to_soil_ranges(&(79..93));
        assert_eq!(soil_ranges.len(), 1);
        assert_eq!(soil_ranges[0], 81..95);
    }
}
