use std::cmp::Ordering;

use aoc2023::utils::read_input_file;
use itertools::Itertools;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents); // 241344943
    println!("part 1: {}", part1);
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

static CARD_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn get_hand_type(hand: &str) -> HandType {
    let card_counts = hand.chars().counts();
    let mut most_freq_count = 0;
    let mut most_freq_char = 'H';
    let mut second_most_freq_count = 0;
    let mut _second_most_freq_char = 'H';
    for (card, count) in card_counts {
        if count > most_freq_count {
            second_most_freq_count = most_freq_count;
            _second_most_freq_char = most_freq_char;
            most_freq_count = count;
            most_freq_char = card;
        } else if count > second_most_freq_count {
            _second_most_freq_char = card;
            second_most_freq_count = count;
        }
    }
    match most_freq_count {
        5 => HandType::FiveKind,
        4 => HandType::FourKind,
        3 => {
            if second_most_freq_count == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeKind
            }
        }
        2 => {
            if second_most_freq_count == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        _ => HandType::HighCard,
    }
}

fn part1(contents: String) -> usize {
    let mut hands_bids: Vec<(&str, usize)> = contents
        .split('\n')
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let mut parts = x.split(' ');
            (
                parts.next().unwrap(),
                parts.next().map(|x| x.parse::<usize>().unwrap()).unwrap(),
            )
        })
        .collect();

    hands_bids.sort_unstable_by(|(l, _), (r, _)| {
        let left_hand_type = get_hand_type(l);
        let right_hand_type = get_hand_type(r);
        if left_hand_type > right_hand_type {
            Ordering::Less
        } else if left_hand_type == right_hand_type {
            for (left_card, right_card) in l.chars().zip(r.chars()) {
                let left_lead_value = CARD_ORDER.iter().position(|x| *x == left_card).unwrap();
                let right_lead_value = CARD_ORDER.iter().position(|x| *x == right_card).unwrap();
                let compare_result = left_lead_value.cmp(&right_lead_value);
                if compare_result != Ordering::Equal {
                    return compare_result;
                }
            }
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    println!("{:?}", hands_bids);
    hands_bids
        .iter()
        .enumerate()
        .fold(0, |total, (rank, (_, bid))| total + bid * (rank + 1))
}

fn part2(contents: String) -> usize {
    contents.split('\n').for_each(|_| ());
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    const P1SAMPLE01_ANSWER: usize = 6440;
    const P2SAMPLE01_ANSWER: usize = 0;

    #[test]
    fn p1sample01() {
        let contents = read_input_file(file!(), "sample.txt");
        let res = part1(contents);
        assert_eq!(res, P1SAMPLE01_ANSWER);
    }
    #[test]
    fn p2sample01() {
        let contents = read_input_file(file!(), "sample.txt");
        let res = part2(contents);
        assert_eq!(res, P2SAMPLE01_ANSWER);
    }
}
