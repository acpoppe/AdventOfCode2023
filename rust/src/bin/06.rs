use advent_of_code::aoc_helpers;
use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = aoc_helpers::read_lines(input);
    let times = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let records = lines[1]
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let mut races: Vec<Race> = vec![];

    for (index, time) in times.iter().enumerate() {
        races.push(Race {
            time: *time,
            record: records[index],
        })
    }

    let mut winning_possibilities_count: Vec<u64> = vec![];

    for race in races.iter() {
        let poss = get_winning_possibilities(&race);
        let range = (poss.0 + 1)..poss.1;
        winning_possibilities_count.push(range.count() as u64);
    }

    Some(winning_possibilities_count.iter().product::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = aoc_helpers::read_lines(input);
    let time = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec()
        .into_iter()
        .join("")
        .parse::<u64>();

    let record = lines[1]
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec()
        .into_iter()
        .join("")
        .parse::<u64>();

    let race = Race {
        time: time.unwrap(),
        record: record.unwrap(),
    };

    let poss = get_winning_possibilities(&race);
    let range = (poss.0 + 1)..poss.1;
    Some(range.count() as u64)
}

fn get_winning_possibilities(race: &Race) -> (i64, i64) {
    let a: f64 = 1.0;
    let b: f64 = race.time as f64;
    let c: f64 = race.record as f64;

    let pos = (-b + (b.powf(2.0) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let neg = (-b - (b.powf(2.0) - 4.0 * a * c).sqrt()) / (2.0 * a);

    (-1 * (pos.ceil() as i64), -1 * (neg.floor() as i64))
}

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
