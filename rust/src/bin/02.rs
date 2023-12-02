advent_of_code::solution!(2);
use advent_of_code::aoc_helpers;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = aoc_helpers::read_lines(input);
    let mut games: Vec<Game> = vec![];
    for (index, game) in lines.enumerate() {
        games.push(parse_game(game, (index + 1) as u32));
    }
    let mut total: u32 = 0;
    for game in games {
        if game.is_possible(12, 13, 14) {
            total += game.index;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = aoc_helpers::read_lines(input);
    let mut games: Vec<Game> = vec![];
    for (index, game) in lines.enumerate() {
        games.push(parse_game(game, (index + 1) as u32));
    }
    let mut total: u32 = 0;
    for game in games {
        let min_counts = game.min_possible_values();
        let power = min_counts.0 * min_counts.1 * min_counts.2;
        total += power;
    }
    Some(total)
}

fn parse_game(from: &str, i: u32) -> Game {
    let game_parts: Vec<&str> = from.split(": ").collect();
    let pull_strings = game_parts[1].split("; ");
    let mut pulls: Vec<Pull> = vec![];
    for pull in pull_strings {
        let cubes = parse_pull(pull);
        pulls.push(cubes);
    }
    Game {
        index: i,
        pulls
    }
}

fn parse_pull(from: &str) -> Pull {
    let colors = from.split(", ");
    let mut red_count: u32 = 0;
    let mut blue_count: u32 = 0;
    let mut green_count: u32 = 0;
    for color in colors {
        match parse_cube_color(color) {
            CubeColor::Blue => blue_count = color.split(" ").collect::<Vec<&str>>()[0].parse().expect("Blue color not found"),
            CubeColor::Red => red_count = color.split(" ").collect::<Vec<&str>>()[0].parse().expect("Red color not found"),
            CubeColor::Green => green_count = color.split(" ").collect::<Vec<&str>>()[0].parse().expect("Green color not found")
        }
    }
    Pull {
        red_count,
        green_count,
        blue_count
    }
}

fn parse_cube_color(from: &str) -> CubeColor {
    if from.ends_with("blue") {
        return CubeColor::Blue;
    } else if from.ends_with("red") {
        return CubeColor::Red;
    }
    CubeColor::Green
}

#[derive(Debug)]
struct Pull {
    red_count: u32,
    green_count: u32,
    blue_count: u32
}

#[derive(Debug)]
struct Game {
    index: u32,
    pulls: Vec<Pull>
}

impl Game {
    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        let mut is_possible = true;
        for pull in &self.pulls {
            if red < pull.red_count || blue < pull.blue_count || green < pull.green_count {
                is_possible = false;
                break;
            }
        }
        is_possible
    }

    fn min_possible_values(&self) -> (u32, u32, u32) {
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;
        for pull in &self.pulls {
            if r < pull.red_count {
                r = pull.red_count;
            }
            if g < pull.green_count {
                g = pull.green_count;
            }
            if b < pull.blue_count {
                b = pull.blue_count;
            }
        }
        (r, g, b)
    }
}

enum CubeColor {
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
