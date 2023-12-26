use advent_of_code::aoc_helpers;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let map = aoc_helpers::read_chars(input);
    let mut galaxies = get_galaxies(&map);
    expand_map(&map, &mut galaxies, false);
    let pairs = create_pairs(&galaxies);
    Some(distance_sum(pairs))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = aoc_helpers::read_chars(input);
    let mut galaxies = get_galaxies(&map);
    expand_map(&map, &mut galaxies, true);
    let pairs = create_pairs(&galaxies);
    Some(distance_sum(pairs))
}

fn get_galaxies(map: &Vec<Vec<char>>) -> Vec<(u64, u64)> {
    let mut galaxies = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if map[y][x] == '#' {
                galaxies.push((x as u64, y as u64));
            }
        }
    }
    galaxies
}

fn expand_map(map: &Vec<Vec<char>>, galaxies: &mut Vec<(u64, u64)>, is_part_two: bool) {
    for idx in (0..map.len()).rev() {
        if map[idx].iter().all(|x| *x == '.') {
            for galaxy_index in 0..galaxies.len() {
                if galaxies[galaxy_index].1 >= idx as u64 {
                    if is_part_two {
                        galaxies[galaxy_index].1 += 999999;
                    } else {
                        galaxies[galaxy_index].1 += 1;
                    }
                }
            }
        }
    }

    for idx in (0..map[0].len()).rev() {
        if map.iter().all(|x| x[idx] == '.') {
            for galaxy_index in 0..galaxies.len() {
                if galaxies[galaxy_index].0 >= idx as u64 {
                    if is_part_two {
                        galaxies[galaxy_index].0 += 999999;
                    } else {
                        galaxies[galaxy_index].0 += 1;
                    }
                }
            }
        }
    }
}

fn create_pairs(galaxies: &Vec<(u64, u64)>) -> Vec<Vec<(u64, u64)>> {
    let mut pairs = Vec::new();
    for idx in 0..galaxies.len() {
        if idx < galaxies.len() - 1 {
            for idx2 in (idx + 1)..galaxies.len() {
                pairs.push(vec![galaxies[idx], galaxies[idx2]]);
            }
        }
    }
    pairs
}

fn distance_sum(galaxy_pairs: Vec<Vec<(u64, u64)>>) -> u64 {
    let mut sum = 0;
    for pair in galaxy_pairs {
        sum += ((pair[0].0 as i64 - pair[1].0 as i64).abs()
            + (pair[0].1 as i64 - pair[1].1 as i64).abs()) as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
