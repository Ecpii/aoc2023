use aoc2023::utils::read_input_file;
use array2d::Array2D;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 33728
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn find_mirror_row(pattern: &Array2D<char>) -> Option<usize> {
    (1..pattern.num_rows()).find(|&y| check_mirror_row(pattern, y))
}

fn find_mirror_row_excluding(pattern: &Array2D<char>, excluded_index: usize) -> Option<usize> {
    (1..pattern.num_rows()).find(|&y| y != excluded_index && check_mirror_row(pattern, y))
}

fn check_mirror_row(pattern: &Array2D<char>, row_index: usize) -> bool {
    let mut backtrack_index = row_index - 1;
    let mut forward_index = row_index;
    loop {
        let backtrack_row = match pattern.row_iter(backtrack_index) {
            Ok(row) => row,
            Err(_) => break,
        };
        let forward_row = match pattern.row_iter(forward_index) {
            Ok(row) => row,
            Err(_) => break,
        };

        if !backtrack_row.eq(forward_row) {
            return false;
        }
        backtrack_index = backtrack_index.wrapping_sub(1);
        forward_index += 1;
    }
    true
}

fn find_mirror_col(pattern: &Array2D<char>) -> Option<usize> {
    (1..pattern.num_columns()).find(|&x| check_mirror_col(pattern, x))
}

fn find_mirror_col_excluding(pattern: &Array2D<char>, excluded_index: usize) -> Option<usize> {
    (1..pattern.num_columns()).find(|&x| x != excluded_index && check_mirror_col(pattern, x))
}

fn check_mirror_col(pattern: &Array2D<char>, col_index: usize) -> bool {
    let mut backtrack_index = col_index - 1;
    let mut forward_index = col_index;
    loop {
        let backtrack_col = match pattern.column_iter(backtrack_index) {
            Ok(row) => row,
            Err(_) => break,
        };
        let forward_col = match pattern.column_iter(forward_index) {
            Ok(row) => row,
            Err(_) => break,
        };

        if !backtrack_col.eq(forward_col) {
            return false;
        }
        backtrack_index = backtrack_index.wrapping_sub(1);
        forward_index += 1;
    }
    true
}

fn part1(contents: String) -> isize {
    let patterns = contents.split("\n\n").take_while(|x| !x.is_empty());
    let mut res = 0;
    for raw_pattern in patterns {
        let height = raw_pattern.matches('\n').count() + 1;
        let width = raw_pattern.find('\n').unwrap();
        let pattern = Array2D::from_iter_row_major(
            raw_pattern.split('\n').flat_map(|x| x.chars()),
            height,
            width,
        )
        .expect("Creating 2D array for pattern failed!");

        if let Some(mirror_row) = find_mirror_row(&pattern) {
            res += mirror_row * 100;
            continue;
        } else if let Some(mirror_col) = find_mirror_col(&pattern) {
            res += mirror_col;
            continue;
        }
    }
    res as isize
}

fn part2(contents: String) -> isize {
    let patterns = contents.split("\n\n").take_while(|x| !x.is_empty());
    let mut res = 0;
    for raw_pattern in patterns {
        let height = raw_pattern.matches('\n').count() + 1;
        let width = raw_pattern.find('\n').unwrap();
        let pattern = Array2D::from_iter_row_major(
            raw_pattern.split('\n').flat_map(|x| x.chars()),
            height,
            width,
        )
        .expect("Creating 2D array for pattern failed!");

        let original_reflection_score = if let Some(mirror_row) = find_mirror_row(&pattern) {
            mirror_row * 100
        } else if let Some(mirror_col) = find_mirror_col(&pattern) {
            mirror_col
        } else {
            panic!("No original reflection line found!")
        };

        for (y, x) in pattern.indices_row_major() {
            let mut altered_pattern = pattern.clone();
            let original_marking = pattern.get(y, x).unwrap();
            let altered_marking = if *original_marking == '#' { '.' } else { '#' };
            altered_pattern.set(y, x, altered_marking).unwrap();

            let new_reflection_score = if let Some(mirror_row) =
                find_mirror_row_excluding(&altered_pattern, original_reflection_score / 100)
            {
                mirror_row * 100
            } else if let Some(mirror_col) =
                find_mirror_col_excluding(&altered_pattern, original_reflection_score)
            {
                mirror_col
            } else {
                continue;
            };
            if new_reflection_score != original_reflection_score {
                res += new_reflection_score;
                break;
            }
        }
    }
    res as isize
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
            expected_out: 405,
        }
        .run()
    }

    #[test]
    fn first_sample_part_two() {
        Sample {
            input_file: "sample.txt",
            part_num: 2,
            expected_out: 400,
        }
        .run()
    }
}
