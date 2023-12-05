pub fn read_lines(input: &str) -> Vec<&str> {
    input.trim().split("\n").collect()
}

pub fn read_chars(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<String> = input.trim().split("\n").map(|s| s.to_string()).collect();
    let c: Vec<Vec<char>> = lines.into_iter().map(|l| l.chars().collect()).collect();
    c
}

pub fn read_sections(input: &str) -> Vec<&str> {
    input.trim().split("\n\n").collect()
}
