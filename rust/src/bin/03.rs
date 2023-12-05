use std::collections::HashMap;

use advent_of_code::aoc_helpers;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = aoc_helpers::read_chars(input);
    let results = &lines
        .iter()
        .enumerate()
        .map(|(i, l)| parse_line(&l, i, &lines).0)
        .collect::<Vec<Vec<u32>>>()
        .into_iter()
        .flat_map(|v| v)
        .collect::<Vec<u32>>();
    Some(results.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    let lines = aoc_helpers::read_chars(_input);
    let gear_maps = &lines
        .iter()
        .enumerate()
        .map(|(i, l)| parse_line(&l, i, &lines).1)
        .collect::<Vec<HashMap<String, Vec<u32>>>>();

    let mut gears: HashMap<String, Vec<u32>> = HashMap::new();
    for map in gear_maps {
        for (gear, values) in map {
            if gears.contains_key(gear) {
                gears.get_mut(gear).unwrap().append(&mut values.clone());
            } else {
                gears.insert(gear.clone(), values.clone());
            }
        }
    }
    let total: u32 = gears
        .iter()
        .map(|(_, v)| if v.len() == 2 { v[0] * v[1] } else { 0 })
        .sum();
    Some(total)
}

fn parse_line(
    line: &Vec<char>,
    row_index: usize,
    chart: &Vec<Vec<char>>,
) -> (Vec<u32>, HashMap<String, Vec<u32>>) {
    let mut ret: Vec<u32> = vec![];
    let mut ret_gears: HashMap<String, Vec<u32>> = HashMap::new();
    let mut current_number: Vec<u32> = vec![];
    let mut is_adjacent = false;
    let mut adjacent_gears: HashMap<String, bool> = HashMap::new();

    for (col_index, char) in line.iter().enumerate() {
        if !char.is_digit(10) {
            if current_number.len() > 0 && is_adjacent {
                ret.push(parse_number(&current_number));
                for (gear, _) in &adjacent_gears {
                    if ret_gears.contains_key(gear) {
                        ret_gears
                            .get_mut(gear)
                            .unwrap()
                            .push(parse_number(&current_number));
                    } else {
                        ret_gears.insert(gear.clone(), vec![parse_number(&current_number)]);
                    }
                }
                adjacent_gears.clear();
            }
            current_number.clear();
            is_adjacent = false;
            continue;
        }
        current_number.push(char.to_digit(10).unwrap());
        let gears = check_for_adjacent_gear(row_index, col_index, chart);
        if gears.len() > 0 {
            for (gear, _) in gears {
                adjacent_gears.insert(gear, true);
            }
        }
        if check_for_adjacent_special_character(row_index, col_index, chart) {
            is_adjacent = true;
        }
    }
    if current_number.len() > 0 && is_adjacent {
        ret.push(parse_number(&current_number));
        for (gear, _) in &adjacent_gears {
            if ret_gears.contains_key(gear) {
                ret_gears
                    .get_mut(gear)
                    .unwrap()
                    .push(parse_number(&current_number));
            } else {
                ret_gears.insert(gear.clone(), vec![parse_number(&current_number)]);
            }
        }
        adjacent_gears.clear();
    }

    (ret, ret_gears)
}

fn check_for_adjacent_gear(
    row: usize,
    col: usize,
    chart: &Vec<Vec<char>>,
) -> HashMap<String, bool> {
    let mut found_gears: HashMap<String, bool> = HashMap::new();
    if row > 0 {
        if col > 0 {
            if gear_at(row - 1, col - 1, chart).is_some() {
                found_gears.insert(gear_at(row - 1, col - 1, chart).unwrap(), true);
            }
        }
        if col < chart[row].len() - 1 {
            if gear_at(row - 1, col + 1, chart).is_some() {
                found_gears.insert(gear_at(row - 1, col + 1, chart).unwrap(), true);
            }
        }
        if gear_at(row - 1, col, chart).is_some() {
            found_gears.insert(gear_at(row - 1, col, chart).unwrap(), true);
        }
    }
    if row < chart.len() - 1 {
        if col > 0 {
            if gear_at(row + 1, col - 1, chart).is_some() {
                found_gears.insert(gear_at(row + 1, col - 1, chart).unwrap(), true);
            }
        }
        if col < chart[row].len() - 1 {
            if gear_at(row + 1, col + 1, chart).is_some() {
                found_gears.insert(gear_at(row + 1, col + 1, chart).unwrap(), true);
            }
        }
        if gear_at(row + 1, col, chart).is_some() {
            found_gears.insert(gear_at(row + 1, col, chart).unwrap(), true);
        }
    }
    if col > 0 {
        if gear_at(row, col - 1, chart).is_some() {
            found_gears.insert(gear_at(row, col - 1, chart).unwrap(), true);
        }
    }
    if col < chart[row].len() - 1 {
        if gear_at(row, col + 1, chart).is_some() {
            found_gears.insert(gear_at(row, col + 1, chart).unwrap(), true);
        }
    }
    found_gears
}

fn gear_at(row: usize, col: usize, chart: &Vec<Vec<char>>) -> Option<String> {
    if chart[row][col] == '*' {
        return Some(format!("{}-{}", row, col));
    }
    None
}

fn check_for_adjacent_special_character(row: usize, col: usize, chart: &Vec<Vec<char>>) -> bool {
    let mut is_special = false;
    if row > 0 {
        if col > 0 {
            is_special = is_special_char_at(row - 1, col - 1, chart) || is_special;
        }
        if col < chart[row].len() - 1 {
            is_special = is_special_char_at(row - 1, col + 1, chart) || is_special;
        }
        is_special = is_special_char_at(row - 1, col, chart) || is_special;
    }
    if row < chart.len() - 1 {
        if col > 0 {
            is_special = is_special_char_at(row + 1, col - 1, chart) || is_special;
        }
        if col < chart[row].len() - 1 {
            is_special = is_special_char_at(row + 1, col + 1, chart) || is_special;
        }
        is_special = is_special_char_at(row + 1, col, chart) || is_special;
    }
    if col > 0 {
        is_special = (is_special_char_at(row, col - 1, chart) && !chart[row][col - 1].is_digit(10))
            || is_special;
    }
    if col < chart[row].len() - 1 {
        is_special = (is_special_char_at(row, col + 1, chart) && !chart[row][col + 1].is_digit(10))
            || is_special;
    }
    is_special
}

fn is_special_char_at(row: usize, col: usize, chart: &Vec<Vec<char>>) -> bool {
    chart[row][col] != '.'
}

fn parse_number(digits: &Vec<u32>) -> u32 {
    let mut ret: u32 = 0;
    let mut current_digit: u32 = 0;

    for digit in digits.iter().rev() {
        if current_digit > 0 {
            ret += digit * (10u32.pow(current_digit));
        } else {
            ret += digit;
        }
        current_digit += 1;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
