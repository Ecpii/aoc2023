use std::collections::HashSet;

use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 20107
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn part1(contents: String) -> usize {
    contents
        .split('\n')
        .take_while(|x| !x.is_empty())
        .fold(0, |total_points, current_card| {
            let mut number_sections = current_card.split(": ").nth(1).unwrap().split(" | ");
            let winning_number_section = number_sections.next().unwrap();
            let owned_number_section = number_sections.next().unwrap();

            let mut winning_numbers = HashSet::new();
            winning_number_section.split_whitespace().for_each(|x| {
                winning_numbers.insert(x);
            });

            let mut owned_numbers = HashSet::new();
            owned_number_section.split_whitespace().for_each(|x| {
                owned_numbers.insert(x);
            });

            if matches!(winning_numbers.intersection(&owned_numbers).next(), None) {
                return total_points;
            }

            let score = 1
                << (winning_numbers
                    .intersection(&owned_numbers)
                    .fold(0usize, |x, _| x + 1)
                    - 1);
            total_points + score
        })
}

fn part2(contents: String) -> usize {
    contents.split('\n').for_each(|_| ());
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    const P1SAMPLE01_ANSWER: usize = 13;
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
