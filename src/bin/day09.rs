#![feature(iter_map_windows)]

use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1);
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn part1(contents: String) -> isize {
    let mut res = 0;

    for line in contents.split('\n').take_while(|x| !x.is_empty()) {
        let mut current_seq: Vec<isize> = line
            .split(' ')
            .map(|x| x.parse::<isize>().unwrap())
            .collect();

        let mut last_numbers: Vec<isize> = Vec::with_capacity(current_seq.len());
        last_numbers.push(*current_seq.last().unwrap());
        while !current_seq.iter().all(|x| *x == 0) {
            current_seq = current_seq.iter().map_windows(|[x, y]| *y - *x).collect();
            last_numbers.push(*current_seq.last().unwrap());
        }
        let next_value = last_numbers.iter().sum::<isize>();
        res += next_value;
    }
    res
}

fn part2(contents: String) -> isize {
    let mut res = 0;

    for line in contents.split('\n').take_while(|x| !x.is_empty()) {
        let mut current_seq: Vec<isize> = line
            .split(' ')
            .map(|x| x.parse::<isize>().unwrap())
            .collect();

        let mut first_numbers: Vec<isize> = Vec::with_capacity(current_seq.len());
        first_numbers.push(*current_seq.first().unwrap());
        while !current_seq.iter().all(|x| *x == 0) {
            current_seq = current_seq.iter().map_windows(|[x, y]| *y - *x).collect();
            first_numbers.push(*current_seq.first().unwrap());
        }
        let next_value = first_numbers.iter().rev().fold(0, |acc, new| new - acc);
        dbg!(&next_value);
        res += next_value;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    const P1SAMPLE01_ANSWER: isize = 114;
    const P2SAMPLE01_ANSWER: isize = 2;

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
