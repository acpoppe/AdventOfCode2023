use advent_of_code::aoc_helpers;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = aoc_helpers::read_lines(input);
    let val = lines
        .into_iter()
        .map(|line| parse_winning_num_count(line))
        .map(|n| if n > 0 { 1 << (n - 1) } else { 0 })
        .sum();
    Some(val)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = aoc_helpers::read_lines(input);
    let vals: Vec<u32> = lines
        .into_iter()
        .map(|line| parse_winning_num_count(line))
        .collect();

    let mut bonus_cards: HashMap<u32, u32> = HashMap::new();
    for (index, val) in vals.iter().enumerate() {
        let index: u32 = index.try_into().unwrap();
        let current_count = bonus_cards.get(&index).unwrap_or(&0) + 1;
        for i in 1..(val + 1) {
            let i: u32 = i.try_into().unwrap();
            bonus_cards.insert(
                index + i,
                bonus_cards.get(&(index + i)).unwrap_or(&0) + current_count,
            );
        }
    }
    let line_count: u32 = vals.len() as u32;
    Some(bonus_cards.values().sum::<u32>() + line_count)
}

fn parse_winning_num_count(line: &str) -> u32 {
    let nums = line.split_once(": ").unwrap();
    let (winning_nums, card_vals): (HashSet<u32>, Vec<u32>) = nums
        .1
        .split_once(" | ")
        .map(|(w, n)| {
            (
                w.split_whitespace()
                    .map(|w| w.parse::<u32>().unwrap())
                    .collect(),
                n.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect(),
            )
        })
        .unwrap();
    card_vals
        .iter()
        .filter(|n| winning_nums.contains(n))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
