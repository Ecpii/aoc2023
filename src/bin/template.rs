use std::time::Instant;

use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let start = Instant::now();
    let part1 = part1(contents);
    let duration = start.elapsed();
    println!("part 1: {}", part1);
    println!("part 1 took {:?}", duration);
    let contents = read_input_file(file!(), "input.txt");
    let start2 = Instant::now();
    let part2 = part2(contents);
    let duration2 = start2.elapsed();
    println!("part 2: {}", part2);
    println!("part 2 took {:?}", duration2);
}

fn part1(contents: String) -> isize {
    let lines = contents.split('\n').take_while(|x| !x.is_empty());
    let mut res = 0;
    res
}

fn part2(contents: String) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};
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
    fn first_sample() {
        Sample {
            input_file: "sample.txt",
            part_num: 1,
            expected_out: 0,
        }
        .run()
    }

    #[test]
    fn first_sample_part_two() {
        Sample {
            input_file: "sample.txt",
            part_num: 2,
            expected_out: 0,
        }
        .run()
    }
}
