use advent_of_code::aoc_helpers;
use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = aoc_helpers::read_lines(input);
    let mut hands = lines.iter().map(|line| Hand::new(line)).collect_vec();

    hands.sort();
    hands.reverse();

    Some(hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + hand.bid * ((index as u64) + 1)
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = aoc_helpers::read_lines(input);
    let mut hands = lines.iter().map(|line| HandPt2::new(line)).collect_vec();

    hands.sort();
    hands.reverse();

    Some(hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + hand.bid * ((index as u64) + 1)
    }))
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
    hand_type: HandType,
}

impl Hand {
    fn new(line: &str) -> Hand {
        let parts: Vec<&str> = line.split_whitespace().collect();
        Hand {
            cards: parts[0].chars().map(|c| Card::parse_card(c)).collect_vec(),
            bid: parts[1].parse::<u64>().unwrap(),
            hand_type: HandType::parse_hand(&parts[0].chars().collect()),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (index, card) in self.cards.iter().enumerate() {
                if card != &other.cards[index] {
                    return card.cmp(&other.cards[index]);
                }
            }
            self.bid.cmp(&other.bid)
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct HandPt2 {
    cards: Vec<CardPt2>,
    bid: u64,
    hand_type: HandType,
}

impl HandPt2 {
    fn new(line: &str) -> HandPt2 {
        let parts: Vec<&str> = line.split_whitespace().collect();
        HandPt2 {
            cards: parts[0]
                .chars()
                .map(|c| CardPt2::parse_card(c))
                .collect_vec(),
            bid: parts[1].parse::<u64>().unwrap(),
            hand_type: HandType::parse_hand_pt_2(&parts[0].chars().collect()),
        }
    }
}

impl Ord for HandPt2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (index, card) in self.cards.iter().enumerate() {
                if card != &other.cards[index] {
                    return card.cmp(&other.cards[index]);
                }
            }
            self.bid.cmp(&other.bid)
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for HandPt2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn parse_card(card: char) -> Self {
        match card {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CardPt2 {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

impl CardPt2 {
    fn parse_card(card: char) -> Self {
        match card {
            'A' => CardPt2::A,
            'K' => CardPt2::K,
            'Q' => CardPt2::Q,
            'J' => CardPt2::J,
            'T' => CardPt2::T,
            '9' => CardPt2::Nine,
            '8' => CardPt2::Eight,
            '7' => CardPt2::Seven,
            '6' => CardPt2::Six,
            '5' => CardPt2::Five,
            '4' => CardPt2::Four,
            '3' => CardPt2::Three,
            '2' => CardPt2::Two,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn parse_hand(cards: &Vec<char>) -> Self {
        let mut counts = std::collections::HashMap::new();
        for card in cards {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }
        let mut highest_count = 0;
        for count in counts.clone() {
            if count.1 > highest_count {
                highest_count = count.1;
            }
        }
        if highest_count == 3 {
            for count in counts.clone() {
                if count.1 == 2 {
                    return HandType::FullHouse;
                }
            }
        }
        if highest_count == 2 {
            let mut pair_count = 0;
            for count in counts {
                if count.1 == 2 {
                    pair_count += 1;
                }
            }
            if pair_count == 2 {
                return HandType::TwoPair;
            }
        }
        match highest_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => HandType::ThreeOfAKind,
            2 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn parse_hand_pt_2(cards: &Vec<char>) -> Self {
        let mut counts = std::collections::HashMap::new();
        for card in cards {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }
        let mut highest_count = 0;
        for count in counts.clone() {
            if count.1 + counts.get(&'J').unwrap_or(&0) > highest_count && count.0 != &'J' {
                highest_count = count.1 + counts.get(&'J').unwrap_or(&0);
            } else if count.1 > highest_count {
                highest_count = count.1;
            }
        }
        if highest_count == 3 {
            for count in counts.clone() {
                if count.1 >= (*counts.get(&'J').unwrap_or(&0)) {
                    if count.1 == 2 && counts.get(&'J').unwrap_or(&0) == &0 {
                        return HandType::FullHouse;
                    } else if count.1 == 2 && counts.get(&'J').unwrap_or(&0) == &1 {
                        let mut pair_count = 0;
                        for count in counts.clone() {
                            if count.1 == 2 {
                                pair_count += 1;
                            }
                        }
                        if pair_count == 2 {
                            return HandType::FullHouse;
                        }
                    }
                }
            }
        }
        if highest_count == 2 {
            let mut pair_count = 0;
            let mut jacks_consumed = 0;
            for count in counts.clone() {
                if count.1 >= (*counts.get(&'J').unwrap_or(&0) - jacks_consumed) {
                    if count.1 - (counts.get(&'J').unwrap_or(&0) - jacks_consumed) == 2 {
                        pair_count += 1;
                        jacks_consumed += counts.get(&'J').unwrap_or(&0) - jacks_consumed;
                    }
                }
            }
            if pair_count == 2 {
                return HandType::TwoPair;
            }
        }
        match highest_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => HandType::ThreeOfAKind,
            2 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
