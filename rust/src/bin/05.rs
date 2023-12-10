use std::{collections::HashMap, cmp::{min, max}};
use itertools::Itertools;

use advent_of_code::aoc_helpers;
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let sections = aoc_helpers::read_sections(input);
    let seeds = sections[0];
    let maps = sections[1..].to_vec();

    let seeds: Vec<u64> = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let maps: HashMap<AlmanacKey, Map> = maps
        .iter()
        .map(|s| parse_map(s))
        .collect();

    Some(convert_seeds_to_locations(seeds, &maps).iter().min().unwrap().clone())
}

pub fn part_two(_input: &str) -> Option<u64> {
    let sections = aoc_helpers::read_sections(_input);
    let seeds = sections[0].split_once(": ").unwrap().1;
    let maps: Vec<Vec<Conversion>> = sections[1..]
        .to_vec()
        .iter()
        .map(|s| parse_map_pt2(s).conversions)
        .collect_vec();
    let ranges: Vec<SeedRange> = seeds
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .into_iter()
        .map(|c| {
            SeedRange {
                min: c[0].parse::<u64>().unwrap(),
                length: c[1].parse::<u64>().unwrap(),
            }
        })
        .collect();

    let mapped_ranges = maps
        .iter()
        .fold(ranges, |ranges, map| {
            ranges
                .iter()
                .flat_map(|seed_range| map.convert_ranges_to_locations(seed_range))
                .filter(|r| r.length > 0)
                .sorted()
                .collect_vec()
                .merge_ranges()
        });

    Some(mapped_ranges.iter().map(|r| r.min).min().unwrap().clone())
}

trait Convertable {
    fn convert_ranges_to_locations(&self, range: &SeedRange) -> Vec<SeedRange>;
}

impl Convertable for Vec<Conversion> {
    fn convert_ranges_to_locations(&self, range: &SeedRange) -> Vec<SeedRange> {
        let mut new_ranges = vec![];
        let mut new_start = range.min;
        let mut remaining_length = range.length;
        let mut start_index = match self.binary_search_by(|c| c.source_start.cmp(&range.min)) {
            Ok(i) => i,
            Err(i) => i,
        };
        let earliest = Conversion {
            source_start: 0,
            source_end: 0,
            destination: 0,
            range: 0,
        };
        let latest = Conversion {
            source_start: u64::MAX,
            source_end: u64::MAX,
            destination: u64::MAX,
            range: 0,
        };

        while remaining_length > 0 {
            let previous = if start_index > 0 {
                &self[start_index - 1]
            } else {
                &earliest
            };

            let next = if start_index < self.len() {
                &self[start_index]
            } else {
                &latest
            };

            if previous.includes(new_start) {
                new_ranges.push(SeedRange {
                    min: previous.destination + new_start - previous.source_start,
                    length: min(remaining_length, previous.source_end - new_start),
                });
                remaining_length -= min(remaining_length, previous.source_end - new_start);
                new_start = previous.source_end;
            }

            new_ranges.push(SeedRange {
                min: new_start,
                length: min(remaining_length, next.source_start - new_start),
            });
            remaining_length -= min(remaining_length, next.source_start - new_start);
            new_start = next.source_start;

            start_index += 1;
        }

        new_ranges
    }
}

fn convert_seeds_to_locations(seeds: Vec<u64>, maps: &HashMap<AlmanacKey, Map>) -> Vec<u64> {
    seeds.iter().map(|s| convert_seed_to_location(*s, maps)).collect()
}

fn convert_seed_to_location(seed: u64, maps: &HashMap<AlmanacKey, Map>) -> u64 {
    let val = convert_seed(seed, maps.get(&AlmanacKey::SeedToSoil).unwrap()).unwrap_or(seed);
    let val = convert_seed(val, maps.get(&AlmanacKey::SoilToFertilizer).unwrap()).unwrap_or(val);
    let val = convert_seed(val, maps.get(&AlmanacKey::FertilizerToWater).unwrap()).unwrap_or(val);
    let val = convert_seed(val, maps.get(&AlmanacKey::WaterToLight).unwrap()).unwrap_or(val);
    let val = convert_seed(val, maps.get(&AlmanacKey::LightToTemperature).unwrap()).unwrap_or(val);
    let val = convert_seed(val, maps.get(&AlmanacKey::TemperatureToHumidity).unwrap()).unwrap_or(val);
    let val = convert_seed(val, maps.get(&AlmanacKey::HumidityToLocation).unwrap()).unwrap_or(val);
    val
}

fn convert_seed(seed: u64, map: &Map) -> Option<u64> {
    for conversion in &map.conversions {
        if seed >= conversion.source_start && seed < conversion.source_start + conversion.range {
            for i in conversion.source_start..(conversion.source_start + conversion.range) {
                if i == seed {
                    return Some(conversion.destination + (i - conversion.source_start));
                }
            }
        }
    }
    None
}

fn parse_map(input: &str) -> (AlmanacKey, Map) {
    let lines: Vec<&str> = input.split("\n").collect();
    let key = match lines[0].trim() {
        "seed-to-soil map:" => AlmanacKey::SeedToSoil,
        "soil-to-fertilizer map:" => AlmanacKey::SoilToFertilizer,
        "fertilizer-to-water map:" => AlmanacKey::FertilizerToWater,
        "water-to-light map:" => AlmanacKey::WaterToLight,
        "light-to-temperature map:" => AlmanacKey::LightToTemperature,
        "temperature-to-humidity map:" => AlmanacKey::TemperatureToHumidity,
        "humidity-to-location map:" => AlmanacKey::HumidityToLocation,
        _ => panic!("Unknown key"),
    };

    let conversions: Vec<Conversion> = lines[1..].iter().map(|s| parse_conversion(s)).collect();

    (
        key,
        Map {
            conversions,
        }
    )
}

fn parse_map_pt2(input: &str) -> Map {
    let lines: Vec<&str> = input.split("\n").collect();
    let conversions: Vec<Conversion> = lines[1..].iter().map(|s| parse_conversion(s)).sorted().collect();
    Map {
        conversions,
    }
}

fn parse_conversion(input: &str) -> Conversion {
    let parts: Vec<u64> = input.split_whitespace().map(|p| p.trim().parse::<u64>().unwrap()).collect();
    Conversion {
        source_start: parts[1],
        source_end: parts[1] + parts[2],
        destination: parts[0],
        range: parts[2],
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct SeedRange {
    min: u64,
    length: u64,
}

impl SeedRange {
    fn includes(&self, seed: u64) -> bool {
        seed >= self.min && seed < self.min + self.length
    }
}

trait MergeRanges {
    fn merge_ranges(&self) -> Vec<SeedRange>;
}

impl MergeRanges for Vec<SeedRange> {
    fn merge_ranges(&self) -> Vec<SeedRange> {
        let mut merged_ranges = vec![];
        let mut current_range = self[0].clone();
        self.iter().for_each(|range| {
            if current_range.includes(range.min) {
                current_range.length = max(current_range.length, range.length + range.min - current_range.min);
            } else if range.min == current_range.min + current_range.length {
                current_range.length += range.length;
            } else {
                merged_ranges.push(current_range.clone());
                current_range = range.clone();
            }
        });
        merged_ranges.push(current_range);
        merged_ranges
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Conversion {
    source_start: u64,
    source_end: u64,
    destination: u64,
    range: u64,
}

impl Conversion {
    fn includes(&self, seed: u64) -> bool {
        seed >= self.source_start && seed < self.source_end
    }
}

#[derive(Debug)]
struct Map {
    conversions: Vec<Conversion>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum AlmanacKey {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
