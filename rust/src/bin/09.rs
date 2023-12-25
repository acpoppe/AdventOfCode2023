use advent_of_code::aoc_helpers;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let numbers = aoc_helpers::read_lines(input)
        .into_iter()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>().into_iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>()
        .into_iter()
        .map(|h| predict_val(&h, false))
        .collect::<Vec<i64>>();

    Some(numbers.iter().sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let numbers = aoc_helpers::read_lines(input)
        .into_iter()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>().into_iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>()
        .into_iter()
        .map(|h| predict_val(&h, true))
        .collect::<Vec<i64>>();

    Some(numbers.iter().sum())
}

fn predict_val(historic_vals: &Vec<i64>, prev: bool) -> i64 {
    if historic_vals.iter().all(|v| *v == 0) {
        return 0;
    }

    let mut steps: Vec<i64> = vec![];
    for i in 1..historic_vals.len() {
        steps.push(historic_vals[i] - historic_vals[i - 1]);
    }

    if prev {
        return historic_vals.first().unwrap() - predict_val(&steps, prev);
    }

    historic_vals.last().unwrap() + predict_val(&steps, prev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
