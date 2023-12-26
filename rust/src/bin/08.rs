use futures::future::join_all;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let sections = input.split_once("\n\n")?;
    let steps = sections.0.chars().collect::<Vec<char>>();
    let mut nodes = Nodes {
        nodes: HashMap::new(),
    };

    for line in sections.1.trim().lines() {
        let (name, left, right) = parse_line(line);
        nodes.add(name, (left, right));
    }

    let mut current = "AAA";
    let mut count: u64 = 0;
    loop {
        let step = steps[(count as usize) % steps.len()];
        if step == 'L' {
            current = nodes.get_left(current).unwrap();
            count += 1;
        } else {
            current = nodes.get_right(current).unwrap();
            count += 1;
        }
        if current == "ZZZ" {
            break;
        }
    }
    Some(count)
}

#[tokio::main(flavor = "current_thread")]
pub async fn part_two(input: &str) -> Option<u64> {
    let sections = input.split_once("\n\n")?;
    let steps = sections.0.chars().collect::<Vec<char>>();
    let mut nodes = Nodes {
        nodes: HashMap::new(),
    };
    let starts: Vec<String> = sections
        .1
        .trim()
        .lines()
        .map(|line| parse_line(line).0)
        .filter(|name| name.ends_with("A"))
        .collect();

    for line in sections.1.trim().lines() {
        let (name, left, right) = parse_line(line);
        nodes.add(name, (left, right));
    }

    let mut counts = vec![];
    for start in starts {
        counts.push(find_loop_count(steps.clone(), start, &nodes));
    }
    let counts = join_all(counts).await;
    Some(lcm(counts))
}

async fn find_loop_count(steps: Vec<char>, start: String, nodes: &Nodes) -> u64 {
    let mut current: &str = &start;
    let mut count: u64 = 0;
    loop {
        let step = steps[(count as usize) % steps.len()];
        if step == 'L' {
            current = nodes.get_left(current).unwrap();
            count += 1;
        } else {
            current = nodes.get_right(current).unwrap();
            count += 1;
        }
        if current.ends_with("Z") {
            break;
        }
    }
    count
}

fn parse_line(line: &str) -> (String, String, String) {
    let parts = line.split(" = ").collect::<Vec<&str>>();
    let name = parts[0].to_string();
    let dests = parts[1]
        .split_at(parts[1].len() - 1)
        .0
        .split_at(1)
        .1
        .split_once(", ")
        .unwrap();

    (name, dests.0.to_string(), dests.1.to_string())
}

fn lcm(numbers: Vec<u64>) -> u64 {
    let mut temp = numbers.clone();

    loop {
        let mut same = true;

        for idx in 1..temp.len() {
            if temp[0] != temp[idx] {
                same = false;
                break;
            }
        }

        if same {
            return temp[0];
        }

        match temp
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
        {
            Some(idx) => {
                temp[idx] = temp[idx] + numbers[idx];
            }
            None => panic!("Not possible"),
        }
    }
}

#[derive(Debug)]
struct Nodes {
    nodes: HashMap<String, (String, String)>,
}

impl Nodes {
    fn add(&mut self, name: String, dests: (String, String)) {
        self.nodes.insert(name, dests);
    }

    fn get_left(&self, name: &str) -> Option<&String> {
        self.nodes.get(name).map(|dests| &dests.0)
    }

    fn get_right(&self, name: &str) -> Option<&String> {
        self.nodes.get(name).map(|dests| &dests.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
