use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 9565386
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn distance_from(&self, other: Coord) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn get_blank_col_indices(map: Vec<&str>) -> Vec<usize> {
    let mut res = vec![true; map[0].len()];
    for line in map {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                res[x] = false;
            }
        }
    }

    res.iter()
        .enumerate()
        .filter(|(_, is_empty)| **is_empty)
        .map(|(index, _)| index)
        .collect()
}

fn part1(contents: String) -> isize {
    let lines: Vec<_> = contents.split('\n').take_while(|x| !x.is_empty()).collect();
    let mut stars: Vec<Coord> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                stars.push(Coord { x, y });
            }
        }
    }

    let blank_line_indices: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|x| x == '.'))
        .map(|(index, _)| index)
        .collect();
    let blank_col_indices: Vec<usize> = get_blank_col_indices(lines);

    for star in stars.iter_mut() {
        let num_prev_expanded_lines = blank_line_indices
            .iter()
            .take_while(|y| **y < star.y)
            .count();
        let num_prev_expanded_cols = blank_col_indices
            .iter()
            .take_while(|x| **x < star.x)
            .count();
        star.x += num_prev_expanded_cols;
        star.y += num_prev_expanded_lines;
    }

    let mut res = 0;

    for (index, star) in stars.iter().enumerate() {
        for other_star in stars.iter().skip(index + 1) {
            res += star.distance_from(*other_star)
        }
    }

    res as isize
}

fn part2(contents: String) -> isize {
    let lines: Vec<_> = contents.split('\n').take_while(|x| !x.is_empty()).collect();
    let mut stars: Vec<Coord> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                stars.push(Coord { x, y });
            }
        }
    }

    let blank_line_indices: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|x| x == '.'))
        .map(|(index, _)| index)
        .collect();
    let blank_col_indices: Vec<usize> = get_blank_col_indices(lines);

    for star in stars.iter_mut() {
        let num_prev_expanded_lines = blank_line_indices
            .iter()
            .take_while(|y| **y < star.y)
            .count();
        let num_prev_expanded_cols = blank_col_indices
            .iter()
            .take_while(|x| **x < star.x)
            .count();
        star.x += num_prev_expanded_cols * (1000000 - 1);
        star.y += num_prev_expanded_lines * (1000000 - 1);
    }

    let mut res = 0;

    for (index, star) in stars.iter().enumerate() {
        for other_star in stars.iter().skip(index + 1) {
            res += star.distance_from(*other_star)
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
            expected_out: 374,
        }
        .run()
    }
}
