use std::collections::HashMap;

use advent_of_code::aoc_helpers;
use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let mut memoized: HashMap<String, u64> = HashMap::new();

    Some(
        aoc_helpers::read_lines(input)
            .iter()
            .map(|line| {
                let parts = line.split(" ").collect::<Vec<&str>>();
                let groupings = parts[1]
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                get_variation_count(parts[0].chars().collect(), groupings, &mut memoized)
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut memoized: HashMap<String, u64> = HashMap::new();

    Some(
        aoc_helpers::read_lines(input)
            .iter()
            .map(|line| {
                let parts = line.split(" ").collect::<Vec<&str>>();
                let groupings = parts[1]
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let (line, groupings) = unfold(parts[0].chars().collect(), groupings);
                get_variation_count(line, groupings, &mut memoized)
            })
            .sum::<u64>(),
    )
}

fn unfold(line: Vec<char>, groupings: Vec<u64>) -> (Vec<char>, Vec<u64>) {
    let mut new_line = line.clone();
    let mut new_groupings = groupings.clone();

    for _ in 0..4 {
        let mut line_copy = line.clone();
        let mut groupings_copy = groupings.clone();
        new_line.push('?');
        new_line.append(&mut line_copy);
        new_groupings.append(&mut groupings_copy);
    }
    (new_line, new_groupings)
}

fn get_variation_count(
    line: Vec<char>,
    groupings: Vec<u64>,
    memoized: &mut HashMap<String, u64>,
) -> u64 {
    if line.len() == 0 && groupings.len() == 0 {
        return 1;
    }

    if line.len() == 0 && groupings.len() > 0 {
        return 0;
    }

    if line.first().unwrap() == &'.' {
        let mut key = line[1..].to_vec();
        key.append(
            &mut groupings
                .clone()
                .iter()
                .map(|x| {
                    x.to_string()
                        .chars()
                        .join(&",")
                        .chars()
                        .collect::<Vec<char>>()
                })
                .flatten()
                .collect::<Vec<char>>(),
        );
        let key = key.iter().collect::<String>();
        if memoized.contains_key(&key) {
            return *memoized.get(&key).unwrap();
        }
        let val = get_variation_count(line[1..].to_vec(), groupings, memoized);
        memoized.insert(key, val);
        return val;
    }

    if line.first().unwrap() == &'#' {
        if groupings.len() > 0 && line.len() >= *groupings.first().unwrap() as usize {
            let potential_grouping = line[0..*groupings.first().unwrap() as usize].to_vec();
            if potential_grouping.iter().all(|x| x != &'.') {
                let mut new_line = line[*groupings.first().unwrap() as usize..].to_vec();
                if new_line.len() > 0 && new_line.first().unwrap() == &'?' {
                    new_line[0] = '.';
                }
                if new_line.len() > 0 && new_line.first().unwrap() == &'.' {
                    let mut key = new_line.clone();
                    key.append(
                        &mut groupings[1..]
                            .to_vec()
                            .iter()
                            .map(|x| {
                                x.to_string()
                                    .chars()
                                    .join(&",")
                                    .chars()
                                    .collect::<Vec<char>>()
                            })
                            .flatten()
                            .collect::<Vec<char>>(),
                    );
                    let key = key.iter().collect::<String>();
                    if memoized.contains_key(&key) {
                        return *memoized.get(&key).unwrap();
                    }
                    let val =
                        get_variation_count(new_line.clone(), groupings[1..].to_vec(), memoized);
                    memoized.insert(key, val);
                    return val;
                }
                if new_line.len() == 0 {
                    return get_variation_count(vec![], groupings[1..].to_vec(), memoized);
                }
            }
        }
    }

    if line.first().unwrap() == &'?' {
        let mut first_variation = line.clone();
        let mut second_variation = line.clone();
        first_variation[0] = '.';
        second_variation[0] = '#';
        let mut first_key = first_variation.clone();
        first_key.append(
            &mut groupings
                .clone()
                .iter()
                .map(|x| {
                    x.to_string()
                        .chars()
                        .join(&",")
                        .chars()
                        .collect::<Vec<char>>()
                })
                .flatten()
                .collect::<Vec<char>>(),
        );
        let first_key = first_key.iter().collect::<String>();
        let first_val: u64;
        if memoized.contains_key(&first_key) {
            first_val = *memoized.get(&first_key).unwrap();
        } else {
            first_val = get_variation_count(first_variation.clone(), groupings.clone(), memoized);
            memoized.insert(first_key, first_val);
        }
        let mut second_key = second_variation.clone();
        second_key.append(
            &mut groupings
                .clone()
                .iter()
                .map(|x| {
                    x.to_string()
                        .chars()
                        .join(&",")
                        .chars()
                        .collect::<Vec<char>>()
                })
                .flatten()
                .collect::<Vec<char>>(),
        );
        let second_key = second_key.iter().collect::<String>();
        let second_val: u64;
        if memoized.contains_key(&second_key) {
            second_val = *memoized.get(&second_key).unwrap();
        } else {
            second_val = get_variation_count(second_variation.clone(), groupings.clone(), memoized);
            memoized.insert(second_key, second_val);
        }
        return first_val + second_val;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
