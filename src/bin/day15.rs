use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1);
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn hash(input: &str) -> usize {
    let mut res: usize = 0;
    for char in input.chars() {
        res += char as usize;
        res *= 17;
        res &= 255;
    }
    res
}
fn part1(contents: String) -> isize {
    let clauses = contents.trim_end().split(',');
    let mut res = 0;
    for clause in clauses {
        res += hash(clause);
    }
    res as isize
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
            expected_out: 1320,
        }
        .run()
    }

    #[test]
    fn first_sample_part_two() {
        Sample {
            input_file: "sample.txt",
            part_num: 1,
            expected_out: 0,
        }
        .run()
    }
}
