use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
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

fn tilt_direction(map: &mut Array2D<char>, direction: Direction) {
    let width = map.num_columns();
    let height = map.num_rows();
    let is_vertical = direction == Direction::North || direction == Direction::South;
    let is_inverted = direction == Direction::South || direction == Direction::East;
    for i in 0..(if is_vertical { width } else { height }) {
        let mut rock_line: Vec<char> = if is_vertical {
            map.column_iter(i).unwrap().cloned().collect()
        } else {
            map.row_iter(i).unwrap().cloned().collect()
        };

        if is_inverted {
            rock_line.reverse();
        }
        let mut landing_area = 0;
        for j in 0..(if is_vertical { height } else { width }) {
            let current_rock = rock_line[j];
            if current_rock == '#' {
                landing_area = j + 1
            } else if current_rock == 'O' {
                if landing_area < j {
                    rock_line[j] = '.';
                    rock_line[landing_area] = 'O';
                }
                while rock_line
                    .get(landing_area)
                    .is_some_and(|landing_rock| *landing_rock != '.')
                {
                    landing_area += 1;
                }
            }
        }
        if is_inverted {
            rock_line.reverse();
        }

        for (j, &new_rock) in rock_line.iter().enumerate() {
            if is_vertical {
                map.set(j, i, new_rock).expect("Setting rock line failed");
            } else {
                map.set(i, j, new_rock).expect("Setting rock line failed");
            }
        }
    }
}

fn spin_cycle(map: &mut Array2D<char>) {
    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];
    for direction in directions {
        tilt_direction(map, direction);
    }
}

fn get_north_load(map: &Array2D<char>) -> usize {
    let height = map.num_rows();
    map.enumerate_row_major()
        .fold(0, |sum, ((y, _), &terrain)| {
            if terrain == 'O' {
                sum + height - y
            } else {
                sum
            }
        })
}

fn part2(contents: String) -> isize {
    let lines = contents.split('\n').take_while(|x| !x.is_empty());
    let height = contents.matches('\n').count();
    let width = contents.find('\n').unwrap();
    let mut map =
        Array2D::from_iter_row_major(lines.flat_map(|x| x.chars()), height, width).unwrap();

    let mut seen_states: HashMap<Array2D<char>, usize> = HashMap::new();
    let mut i = 1000000000;
    while i > 0 {
        if let Some(&last_seen_time) = seen_states.get(&map) {
            let cycle_time = last_seen_time - i;
            i %= cycle_time;
            if i == 0 {
                break;
            }
        }
        seen_states.insert(map.clone(), i);
        spin_cycle(&mut map);
        i -= 1;
    }
    get_north_load(&map) as isize
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

    #[test]
    fn first_sample_part_two() {
        Sample {
            input_file: "sample.txt",
            part_num: 2,
            expected_out: 64,
        }
        .run()
    }
}
