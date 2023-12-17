use std::cmp::max;

use aoc2023::utils::read_input_file;
use array2d::Array2D;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1);
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn ways_to_fit(line: &str, group_size: usize) -> usize {
    let mut num_configs = 0;
    for (index, char) in line.chars().enumerate() {
        if char == '?' || char == '#' {
            let group_window = line.get(index..index + group_size);
            let trailing = line.get(index + group_size..);
            if group_window.is_some_and(|window| window.chars().all(|x| x == '?' || x == '#'))
                && (trailing.is_none() // is_none() check may be unnecessary
                    || trailing.is_some_and(|window| !window.chars().any(|x| x == '#')))
            {
                num_configs += 1;
            }
        }
        if char == '#' {
            return num_configs;
        }
    }
    num_configs
}

/// dp idea
///                string length ->
/// array length v 1 2 3 4 5 6 7 ....
///              1
///              2
///              3
///              4
///              5
/// each cell x,y should be equal to
/// max(
///    table[x - c, y - 1] * ways to fit c in s[x - c..x],
///    table[x - c - 1, y - 1] * ways to fit c in s[x - c - 1..x],
///    ...,
///    table[0, y - 1] * ways to fit c in s[..x]
/// )
///
/// where s is string
/// where c is the len of n[y]
fn count_configurations(line: &str, group_sizes: Vec<usize>) -> usize {
    let width = line.len();
    let height = group_sizes.len();
    let mut memo = Array2D::filled_with(0usize, height, width);

    for x in 0..width {
        memo.set(0, x, ways_to_fit(&line[0..x + 1], group_sizes[0]))
            .expect("Setting first row of memo failed!");
    }

    for (y, current_group_size) in group_sizes.iter().enumerate().skip(1) {
        for x in 2..width {
            let mut max_configs = 0;
            for i in 0..x - 1 {
                let previous = memo.get(y - 1, i).unwrap_or(&1);
                if *previous == 0 || line.chars().nth(i + 1).unwrap() == '#' {
                    continue;
                }
                let test_window = &line[i + 2..x + 1];
                let new_ways = ways_to_fit(test_window, *current_group_size);
                max_configs = max(max_configs, previous * new_ways);
            }
            memo.set(y, x, max_configs).expect("Failed to update memo!");
        }
        // println!("{:?}", &memo.as_rows());
    }
    memo[(height - 1, width - 1)]
}

fn part1(contents: String) -> isize {
    let lines = contents.split('\n').take_while(|x| !x.is_empty());
    let mut res = 0;
    for line in lines {
        let mut line_splits = line.split(' ');
        let spring_record = line_splits.next().unwrap();
        let group_sizes: Vec<usize> = line_splits
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let line_configurations = count_configurations(spring_record, group_sizes);
        dbg!(&line_configurations);
        res += line_configurations;
    }
    res as isize
}

fn part2(contents: String) -> isize {
    let _lines = contents.split('\n').take_while(|x| !x.is_empty());
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file, ways_to_fit};
    struct Sample {
        pub input_file: &'static str,
        pub part_num: u8,
        pub expected_out: isize,
    }
    impl Sample {
        pub fn run(&self) {
            let contents = read_input_file(file!(), self.input_file);
            let res = if self.part_num == 1 {
                part1(contents)
            } else {
                part2(contents)
            };
            assert_eq!(res, self.expected_out);
        }
    }

    #[test]
    fn first_line_first_char() {
        assert_eq!(ways_to_fit("?", 1), 1);
        assert_eq!(ways_to_fit("??", 1), 2);
        assert_eq!(ways_to_fit("???", 1), 3);
        assert_eq!(ways_to_fit("???.", 1), 3);
        assert_eq!(ways_to_fit("???.#", 1), 1);
        assert_eq!(ways_to_fit("???.##", 1), 0);
        assert_eq!(ways_to_fit("???.###", 1), 0);
    }

    #[test]
    fn first_line_group() {
        assert_eq!(ways_to_fit("?", 3), 0);
        assert_eq!(ways_to_fit("??", 3), 0);
        assert_eq!(ways_to_fit("???", 3), 1);
        assert_eq!(ways_to_fit("???.", 3), 1);
        assert_eq!(ways_to_fit("???.#", 3), 0);
        assert_eq!(ways_to_fit("???.##", 3), 0);
        assert_eq!(ways_to_fit("???.###", 3), 1);
    }

    #[test]
    fn group_ways_to_fit() {
        assert_eq!(ways_to_fit("?#?....", 3), 1);
        assert_eq!(ways_to_fit("?#??...", 3), 2);
        assert_eq!(ways_to_fit("????...", 3), 2);
        assert_eq!(ways_to_fit("#?#?...", 3), 1);
        assert_eq!(ways_to_fit("#??#...", 3), 0);
    }

    #[test]
    fn first_sample() {
        Sample {
            input_file: "sample.txt",
            part_num: 1,
            expected_out: 21,
        }
        .run()
    }

    #[test]
    fn first_sample_third_line() {
        assert_eq!(part1("?#?#?#?#?#?#?#? 1,3,1,6".to_string()), 1);
    }

    #[test]
    fn first_sample_last_line() {
        assert_eq!(part1("?###???????? 3,2,1".to_string()), 10);
    }
}
