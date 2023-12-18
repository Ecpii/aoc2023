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

fn part1(contents: String) -> isize {
    let lines = contents.split('\n').take_while(|x| !x.is_empty());
    let height = contents.matches('\n').count();
    let width = contents.find('\n').unwrap();
    let mut map =
        Array2D::from_iter_row_major(lines.flat_map(|x| x.chars()), height, width).unwrap();

    let mut res = 0;
    // dbg!(&map);
    for x in 0..width {
        let mut rock_column: Vec<char> = map.column_iter(x).unwrap().cloned().collect();
        let mut landing_area = 0;
        for y in 0..height {
            let current_rock = rock_column[y];
            if current_rock == '#' {
                landing_area = y + 1
            } else if current_rock == 'O' {
                if landing_area < y {
                    rock_column[y] = '.';
                    rock_column[landing_area] = 'O';
                }
                while rock_column
                    .get(landing_area)
                    .is_some_and(|landing_rock| *landing_rock != '.')
                {
                    landing_area += 1
                }
            }
        }
        for (y, &new_rock) in rock_column.iter().enumerate() {
            map.set(y, x, new_rock).expect("Setting rock column failed");
            if new_rock == 'O' {
                res += height - y;
            }
        }
    }
    // dbg!(&map);
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
            expected_out: 136,
        }
        .run()
    }
}
