use std::{
    cmp::{max, min},
    collections::{hash_map::Entry, HashMap},
    time::Instant,
};

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    pub y: isize,
    pub x: isize,
}
impl Coord {
    pub fn shift_direction(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.y -= 1,
            Direction::West => self.x -= 1,
            Direction::South => self.y += 1,
            Direction::East => self.x += 1,
        }
    }
    pub fn neighbor(&self, direction: Direction) -> Coord {
        match direction {
            Direction::North => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::West => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::South => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn new() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }
}

impl From<&str> for Color {
    fn from(hex_code: &str) -> Self {
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16).unwrap();
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16).unwrap();
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16).unwrap();

        Color { r, g, b }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn is_vertical(&self) -> bool {
        *self == Direction::North || *self == Direction::South
    }
    pub fn _perpendicular_to(&self, other: &Direction) -> bool {
        (self.is_vertical() && !other.is_vertical()) || (!self.is_vertical() && other.is_vertical())
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value.chars().next().unwrap() {
            'U' => Self::North,
            'L' => Self::West,
            'D' => Self::South,
            'R' => Self::East,
            _ => panic!("invalid string provided for direction"),
        }
    }
}

struct Lagoon {
    wall_colors: HashMap<Coord, Color>,
    color_directions: HashMap<Color, Direction>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Lagoon {
    pub fn from_str(contents: String) -> Lagoon {
        let mut wall_colors: HashMap<Coord, Color> = HashMap::new();
        let mut color_directions: HashMap<Color, Direction> = HashMap::new();
        let lines = contents.split('\n').take_while(|x| !x.is_empty());
        let mut current_coord = Coord { x: 0, y: 0 };
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;
        // let mut all_colors: Vec<Color> = Vec::new();
        for line in lines {
            let mut line_splits = line.split(' ');
            let direction: Direction = line_splits.next().unwrap().into();
            let steps: usize = line_splits.next().unwrap().parse().unwrap();
            let color: Color = line_splits.next().unwrap()[1..8].into();
            color_directions.insert(color, direction);
            // all_colors.push(color);
            for _ in 0..steps {
                current_coord.shift_direction(direction);
                wall_colors.insert(current_coord, color);
            }
            min_x = min(current_coord.x, min_x);
            max_x = max(current_coord.x, max_x);
            min_y = min(current_coord.y, min_y);
            max_y = max(current_coord.y, max_y);
        }
        // assert!(all_colors.iter().all_unique()); // true
        Lagoon {
            wall_colors,
            color_directions,
            min_x: min_x - 3,
            max_x: max_x + 3,
            min_y: min_y - 3,
            max_y: max_y + 3,
        }
    }

    fn get_direction(&self, c: Coord) -> Option<Direction> {
        self.color_directions
            .get(self.wall_colors.get(&c).unwrap_or(&Color::new()))
            .copied()
    }

    pub fn dig_inside(&mut self) {
        for y in self.min_y..self.max_y + 1 {
            let mut walls_passed: usize = 0;
            let mut last_vertical_join: Option<Direction> = None;
            for x in self.min_x..self.max_x + 1 {
                let coord = Coord { x, y };

                let map_entry = self.wall_colors.entry(coord);
                if let Entry::Vacant(e) = map_entry {
                    if last_vertical_join.is_some() {
                        walls_passed += 1;
                        last_vertical_join = None;
                    }
                    if walls_passed & 1 == 1 {
                        e.insert(Color::new());
                    }
                    continue;
                }

                let new_vertical_join = self.find_vertical_join(coord);
                if new_vertical_join.is_none() {
                    continue;
                }

                if last_vertical_join.is_some() {
                    if last_vertical_join != new_vertical_join {
                        walls_passed += 1;
                    }
                    last_vertical_join = None;
                } else {
                    last_vertical_join = new_vertical_join;
                }
            }
        }
    }

    fn find_vertical_join(&self, coord: Coord) -> Option<Direction> {
        let below = coord.neighbor(Direction::South);
        let above = coord.neighbor(Direction::North);
        if self.get_direction(coord).unwrap().is_vertical() {
            self.get_direction(coord) // incoming
        } else if self
            .get_direction(above)
            .is_some_and(|x| x == Direction::North)
        {
            Some(Direction::South) // outgoing
        } else if self
            .get_direction(below)
            .is_some_and(|x| x == Direction::South)
        {
            Some(Direction::North) // outgoing
        } else {
            None
        }
    }

    pub fn get_area(&self) -> usize {
        self.wall_colors.len()
    }

    pub fn pretty_print(&self) {
        for y in self.min_y..self.max_y + 1 {
            for x in self.min_x..self.max_x + 1 {
                let coord = Coord { x, y };
                if self.wall_colors.contains_key(&coord) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn part1(contents: String) -> isize {
    let mut lagoon = Lagoon::from_str(contents);
    lagoon.pretty_print();
    lagoon.dig_inside();
    println!();
    lagoon.pretty_print();
    lagoon.get_area() as isize
}

fn part2(_contents: String) -> isize {
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
            expected_out: 62,
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
