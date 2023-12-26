use advent_of_code::aoc_helpers;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let map = aoc_helpers::read_chars(input);
    let start = find_start(&map);
    Some(find_furthest_from(start.0, start.1, &map).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = aoc_helpers::read_chars(input);
    let start = find_start(&map);
    let pipe_loop = find_furthest_from(start.0, start.1, &map);
    let map = remove_extra_from_map(pipe_loop.1, &map, find_paths(start.0, start.1, &map).1);

    Some(count_inner_points(&map))
}

fn count_inner_points(map: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let mut containing_pipe_count = 0;
    let mut last_corner: char = ' ';
    for row in map {
        for col in row {
            if *col == '.' {
                if containing_pipe_count % 2 == 1 {
                    count += 1;
                }
            }
            if *col == '|' {
                containing_pipe_count += 1;
            }
            if *col == 'F' || *col == 'L' {
                last_corner = *col;
            }
            if *col == '7' {
                if last_corner == 'L' {
                    containing_pipe_count += 1;
                }
                last_corner = ' ';
            }
            if *col == 'J' {
                if last_corner == 'F' {
                    containing_pipe_count += 1;
                }
                last_corner = ' ';
            }
        }
    }
    count
}

fn find_start(map: &Vec<Vec<char>>) -> (u64, u64) {
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                return (x as u64, y as u64);
            }
        }
    }
    panic!("No start found");
}

fn remove_extra_from_map(
    paths: Vec<Vec<(u64, u64)>>,
    map: &Vec<Vec<char>>,
    start_char: char,
) -> Vec<Vec<char>> {
    let mut new_map: Vec<Vec<char>> = vec![];
    for y in 0..map.len() {
        new_map.push(vec![]);
        for x in 0..map[y].len() {
            if paths_contains_point(&paths, x as u64, y as u64) {
                if map[y][x] == 'S' {
                    new_map[y].push(start_char);
                } else {
                    new_map[y].push(map[y][x]);
                }
            } else {
                new_map[y].push('.');
            }
        }
    }

    new_map
}

fn paths_contains_point(paths: &Vec<Vec<(u64, u64)>>, x: u64, y: u64) -> bool {
    for path in paths {
        if path.contains(&(x, y)) {
            return true;
        }
    }
    false
}

fn find_furthest_from(x: u64, y: u64, map: &Vec<Vec<char>>) -> (u64, Vec<Vec<(u64, u64)>>) {
    let mut step_count = 1;
    let mut paths = find_paths(x, y, map).0;

    while !paths_are_equal(&paths) {
        step_count += 1;
        for path in paths.iter_mut() {
            step(path, map);
        }
    }

    (step_count, paths)
}

fn paths_are_equal(paths: &Vec<Vec<(u64, u64)>>) -> bool {
    let first_path = paths.first().unwrap();
    let last_step = first_path[first_path.len() - 1];
    for path in paths {
        if path.last().unwrap() != &last_step {
            return false;
        }
    }
    true
}

fn step(path: &mut Vec<(u64, u64)>, map: &Vec<Vec<char>>) {
    let last_step = path[path.len() - 1];
    let prev_step = path[path.len() - 2];
    match map[last_step.1 as usize][last_step.0 as usize] {
        '7' => {
            if last_step.0 - 1 == prev_step.0 && last_step.1 == prev_step.1 {
                path.push((last_step.0, last_step.1 + 1));
                return;
            }
            path.push((last_step.0 - 1, last_step.1))
        }
        'J' => {
            if last_step.0 - 1 == prev_step.0 && last_step.1 == prev_step.1 {
                path.push((last_step.0, last_step.1 - 1));
                return;
            }
            path.push((last_step.0 - 1, last_step.1))
        }
        'F' => {
            if last_step.0 == prev_step.0 && last_step.1 + 1 == prev_step.1 {
                path.push((last_step.0 + 1, last_step.1));
                return;
            }
            path.push((last_step.0, last_step.1 + 1))
        }
        'L' => {
            if last_step.0 == prev_step.0 && last_step.1 - 1 == prev_step.1 {
                path.push((last_step.0 + 1, last_step.1));
                return;
            }
            path.push((last_step.0, last_step.1 - 1))
        }
        '-' => {
            if last_step.0 + 1 == prev_step.0 && last_step.1 == prev_step.1 {
                path.push((last_step.0 - 1, last_step.1));
                return;
            }
            path.push((last_step.0 + 1, last_step.1))
        }
        '|' => {
            if last_step.0 == prev_step.0 && last_step.1 + 1 == prev_step.1 {
                path.push((last_step.0, last_step.1 - 1));
                return;
            }
            path.push((last_step.0, last_step.1 + 1))
        }
        _ => panic!("Invalid step"),
    }
}

fn find_paths(x: u64, y: u64, map: &Vec<Vec<char>>) -> (Vec<Vec<(u64, u64)>>, char) {
    let mut paths: Vec<Vec<(u64, u64)>> = Vec::new();
    let north = ['|', '7', 'F'];
    let south = ['|', 'J', 'L'];
    let east = ['-', 'J', '7'];
    let west = ['-', 'F', 'L'];

    let mut is_north = false;
    let mut is_south = false;
    let mut is_east = false;
    let mut is_west = false;

    if y > 0 && north.contains(&map[y as usize - 1][x as usize]) {
        let mut new_path = Vec::new();
        new_path.push((x, y));
        new_path.push((x, y - 1));
        paths.push(new_path);
        is_north = true;
    }

    if y < map[y as usize].len() as u64 - 1 && south.contains(&map[y as usize + 1][x as usize]) {
        let mut new_path = Vec::new();
        new_path.push((x, y));
        new_path.push((x, y + 1));
        paths.push(new_path);
        is_south = true;
    }

    if x > 0 && west.contains(&map[y as usize][x as usize - 1]) {
        let mut new_path = Vec::new();
        new_path.push((x, y));
        new_path.push((x - 1, y));
        paths.push(new_path);
        is_west = true;
    }

    if x < map.len() as u64 - 1 && east.contains(&map[y as usize][x as usize + 1]) {
        let mut new_path = Vec::new();
        new_path.push((x, y));
        new_path.push((x + 1, y));
        paths.push(new_path);
        is_east = true;
    }

    let start_char = if is_north && is_south {
        '|'
    } else if is_east && is_west {
        '-'
    } else if is_north && is_east {
        'L'
    } else if is_north && is_west {
        'J'
    } else if is_south && is_east {
        'F'
    } else if is_south && is_west {
        '7'
    } else {
        panic!("Invalid start")
    };

    (paths, start_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
