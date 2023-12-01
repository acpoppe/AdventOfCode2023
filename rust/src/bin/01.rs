use std::str::Chars;
advent_of_code::solution!(1);

const ALLOWED_WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim().split("\n");
    let mut calibrations: Vec<u32> = vec![];
    for line in lines {
        let mut digits: Vec<u32> = vec![];
        for char in line.chars() {
            if char.is_digit(10) {
                digits.push(char.to_digit(10).expect("Number invalid"));
            }
        }
        calibrations.push(
            (digits.first().expect("No first digit") * 10) + digits.last().expect("No last digit")
        );
    }
    Some(calibrations.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim().split("\n");
    let mut calibrations: Vec<u32> = vec![];
    for line in lines {
        let first = find_first_digit(&line).unwrap();
        let last = find_last_digit(&line).unwrap();

        let val = (first * 10) + last;
        calibrations.push(val);
    }
    Some(calibrations.iter().sum())
}

fn find_first_digit(line: &str) -> Option<u32> {
    find_digit(line.chars(), true)
}

fn find_last_digit(line: &str) -> Option<u32> {
    find_digit(line.chars(), false)
}

fn find_digit(line: Chars, is_first: bool) -> Option<u32> {
    let mut digit = None;
    let mut current_word: Vec<char> = vec![];
    if is_first {
        for c in line {
            current_word.push(c);
            match parse_char(c, &mut current_word, is_first) {
                Some(val) => {
                    digit = Some(val);
                    break;
                }
                None => ()
            }
        }
    } else {
        for c in line.rev() {
            current_word.push(c);
            match parse_char(c, &mut current_word, is_first) {
                Some(val) => {
                    digit = Some(val);
                    break;
                }
                None => ()
            }
        }
    }
    digit
}

fn parse_char(c: char, current_word: &mut Vec<char>, is_first: bool) -> Option<u32> {
    if c.is_digit(10) {
        return Some(c.to_digit(10).expect("Number invalid"))
    } else {
        match handle_new_letter(current_word, c, is_first) {
            Some(int) => {
                return Some(int)
            },
            None => ()
        }
    }
    None
}

fn handle_new_letter(current_word: &mut Vec<char>, c: char, first_digit: bool) -> Option<u32> {
    let str_rep: String;
    if first_digit {
        str_rep = current_word.iter().cloned().collect::<String>();
    } else {
        let mut reversed = current_word.clone();
        reversed.reverse();
        str_rep = String::from_iter(&reversed);
    }
    let mut match_found = false;
    let mut is_full_word = false;
    for word in ALLOWED_WORDS {
        if check_for_inclusion_in_allowed_words(&str_rep, first_digit) {
            match_found = true;
            if word == str_rep {
                is_full_word = true;
            }
        }
    }
    if !match_found {
        let mut str_rep = &str_rep[..str_rep.len() - 1];
        while current_word.iter().count() > 1 {
            current_word.remove(0);
            str_rep = &str_rep[..str_rep.len() - 1];
            if check_for_inclusion_in_allowed_words(str_rep, first_digit) {
                return None
            }
        }
        return None
    }
    if is_full_word {
        current_word.clear();
        current_word.push(c);
        return Some(translate_word_to_digit(&str_rep[..]).unwrap())
    }
    None
}

fn check_for_inclusion_in_allowed_words(val: &str, first_digit: bool) -> bool {
    for word in ALLOWED_WORDS {
        let is_contained: bool;
        if first_digit {
            is_contained = word.starts_with(val);
        } else {
            is_contained = word.ends_with(val);
        }
        if is_contained {
            return true
        }
    }
    false
}

fn translate_word_to_digit(word: &str) -> Result<u32, &str> {
    match word {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        _ => Err("Not translatable")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
