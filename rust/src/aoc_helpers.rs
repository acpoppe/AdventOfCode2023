use std::str::Split;

pub fn read_lines(input: &str) -> Split<&str> {
    input.trim().split("\n")
}

pub fn read_chars(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<String> = input.trim().split("\n").map(|s| s.to_string()).collect();
    let c: Vec<Vec<char>> = lines.into_iter().map(|l| l.chars().collect()).collect();
    c
}
