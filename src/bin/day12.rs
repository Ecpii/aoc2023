use std::time::Instant;

use aoc2023::utils::read_input_file;
use array2d::Array2D;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1);
    let contents = read_input_file(file!(), "input.txt");
    let start = Instant::now();
    let part2 = part2(contents);
    let duration = start.elapsed();
    println!("Time elapsed for part 2: {:?}", duration); // 27.8925323 seconds fastest
    println!("part 2: {}", part2) // 18716325559999
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
/// shoutout to eecs 281 for brining me this solution
///                                       string index right bound (inclusive) ->
/// array index right bound (inclusive) v 0 1 2 3 4 5 6 ....
///                                     0
///                                     1
///                                     2
///                                     3
///                                     4
/// each cell x,y should be equal to the number of ways to fit the [0..y + 1] elements of
/// the array into the substring from 0 to x + 1
///
/// to calculate, linearly scan through array values on the row above,
/// let i be the x index while scanning
/// if character at string index corresponding to i is '#', then the number of configurations
/// must be less than or equal to the cell left of current cell,
/// because any way of fitting the array into the substring would have to use that #
/// in this case, any configurations we find are unique, and so we add to our running total
/// [configurations at (y - 1, i)] * [ways to fit group size at y into substring from i + 2..]
///
/// actually theres no way i can give a good explanation for this sorry it just works
///
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
            let mut cur_configs = 0;
            for i in 0..x - 1 {
                let mut lhs_ways = memo[(y - 1, i)];
                if lhs_ways == 0 {
                    continue;
                }

                let last_lhs_char = line.chars().nth(i).unwrap();
                if last_lhs_char != '#' {
                    let previous_lhs_ways = memo.get(y - 1, i.wrapping_sub(1)).unwrap_or(&0);
                    lhs_ways -= *previous_lhs_ways;
                }

                if lhs_ways == 0 || line.chars().nth(i + 1).unwrap() == '#' {
                    continue;
                }

                let trailing = &line[i + 2..x + 1];
                let trailing_ways = ways_to_fit(trailing, *current_group_size);
                cur_configs += lhs_ways * trailing_ways;
            }
            memo.set(y, x, cur_configs).expect("Failed to update memo!");
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
        // dbg!(&line_configurations);
        res += line_configurations;
    }
    res as isize
}

fn part2(contents: String) -> isize {
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

        let mut unfolded_spring_record = String::from(spring_record);
        let mut unfolded_group_sizes = Vec::with_capacity(group_sizes.len() * 5);
        unfolded_group_sizes.extend(group_sizes.iter());

        for _ in 0..4 {
            unfolded_spring_record.push('?');
            unfolded_spring_record.push_str(spring_record);
            unfolded_group_sizes.extend(group_sizes.iter());
        }
        let line_configurations =
            count_configurations(&unfolded_spring_record, unfolded_group_sizes);
        // dbg!(&line_configurations);
        res += line_configurations;
    }
    res as isize
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
    fn first_sample_part_2() {
        Sample {
            input_file: "sample.txt",
            part_num: 2,
            expected_out: 525152,
        }
        .run()
    }

    #[test]
    fn all_question_marks() {
        dbg!(part2("??????????????? 1,1,1,2,1".to_string()));
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
