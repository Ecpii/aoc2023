use std::{
    cmp::{max, min, Ordering},
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
    time::Instant,
};

use aoc2023::utils::read_input_file;
use itertools::Itertools;

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

    pub fn shifted(&self, direction: Direction, steps: usize) -> Coord {
        let mut dest = *self;
        match direction {
            Direction::North => dest.y -= steps as isize,
            Direction::West => dest.x -= steps as isize,
            Direction::South => dest.y += steps as isize,
            Direction::East => dest.x += steps as isize,
        }
        dest
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

    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
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
impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '3' => Self::North,
            '2' => Self::West,
            '1' => Self::South,
            '0' => Self::East,
            _ => panic!("invalid char provided for direction"),
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

fn part1(contents: String) -> usize {
    let mut lagoon = Lagoon::from_str(contents);
    // lagoon.pretty_print();
    lagoon.dig_inside();
    // println!();
    // lagoon.pretty_print();
    lagoon.get_area()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Corner {
    Northwest,
    Northeast,
    Southwest,
    Southeast,
}

impl Corner {
    pub fn from_directions(first: Direction, second: Direction) -> Self {
        let (vertical, horizontal) = if first.is_vertical() {
            (first.opposite(), second)
        } else {
            (second, first.opposite())
        };
        if vertical == Direction::North {
            if horizontal == Direction::West {
                Corner::Northwest
            } else {
                Corner::Northeast
            }
        } else if horizontal == Direction::West {
            Corner::Southwest
        } else {
            Corner::Southeast
        }
    }

    pub fn faces(&self, d: Direction) -> bool {
        match d {
            Direction::North => *self == Corner::Northwest || *self == Corner::Northeast,
            Direction::West => *self == Corner::Northwest || *self == Corner::Southwest,
            Direction::South => *self == Corner::Southwest || *self == Corner::Southeast,
            Direction::East => *self == Corner::Northeast || *self == Corner::Southeast,
        }
    }

    pub fn is_bypassable_with(&self, other: Self) -> bool {
        (self.faces(Direction::North) && other.faces(Direction::North))
            || self.faces(Direction::South) && other.faces(Direction::South)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CornerPosition {
    pub corner: Corner,
    pub position: Coord,
}

impl Ord for CornerPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let y_comp = self.position.y.cmp(&other.position.y);
        match y_comp {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => match self.position.x.cmp(&other.position.x) {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            },
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for CornerPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct BigLagoon {
    corners: BinaryHeap<CornerPosition>,
    min_x: isize,
    min_y: isize,
}

impl BigLagoon {
    pub fn from_str(contents: String) -> Self {
        let mut corners: BinaryHeap<CornerPosition> =
            BinaryHeap::with_capacity(contents.chars().filter(|&x| x == '\n').count() + 1);
        let lines = contents.split('\n').take_while(|x| !x.is_empty());
        let mut position = Coord { x: 0, y: 0 };
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut last_direction = lines
            .clone()
            .last()
            .unwrap()
            .chars()
            .take_while(|&x| x != ')')
            .last()
            .unwrap()
            .into();
        for line in lines {
            let hex_code = &line.split(' ').nth(2).unwrap()[2..8];
            let steps = usize::from_str_radix(&hex_code[..5], 16).unwrap();
            let direction: Direction = hex_code.chars().last().unwrap().into();
            corners.push(CornerPosition {
                position,
                corner: Corner::from_directions(last_direction, direction),
            });

            min_x = min(position.x, min_x);
            min_y = min(position.y, min_y);
            let destination = position.shifted(direction, steps);
            position = destination;
            last_direction = direction;
        }

        BigLagoon {
            corners,
            min_x,
            min_y,
        }
    }

    fn get_cornerless_line_area(&self, current_vertical_lines: &[isize]) -> usize {
        let mut area = 0;
        let mut x = self.min_x - 1;
        let mut is_inside = false;
        for &new_x in current_vertical_lines {
            if is_inside {
                area += (new_x - x - 1) as usize;
            }
            area += 1;
            is_inside = !is_inside;
            x = new_x;
        }
        area
    }

    fn get_line_area(
        &self,
        current_vertical_lines: &[isize],
        corners_seen: &[CornerPosition],
    ) -> usize {
        let mut area: usize = 0;
        let x_bounds = corners_seen
            .iter()
            .map(|CornerPosition { position, .. }| &position.x)
            .merge(current_vertical_lines.iter());
        let mut seen_x_bounds = HashSet::new();
        let mut x = self.min_x - 1;
        let mut is_inside = false;
        let mut last_seen_corner: Option<Corner> = None;
        for &new_x in x_bounds {
            if seen_x_bounds.contains(&new_x) {
                continue;
            }
            if is_inside || last_seen_corner.is_some() {
                area += usize::try_from(new_x - x - 1).expect("uh oh"); // exclusive area between points
            }
            area += 1; // a poi is on a dug out point

            if let Ok(new_corner_index) =
                corners_seen.binary_search_by(|corner_pos| corner_pos.position.x.cmp(&new_x))
            {
                // encounter a corner
                let new_corner = &corners_seen[new_corner_index];
                if let Some(old_corner) = last_seen_corner {
                    if !old_corner.is_bypassable_with(new_corner.corner) {
                        is_inside = !is_inside;
                    }
                    last_seen_corner = None;
                } else {
                    last_seen_corner = Some(new_corner.corner);
                };
            } else {
                // encounter a regular vertical line
                is_inside = !is_inside;
            }
            seen_x_bounds.insert(new_x);
            x = new_x;
        }

        area
    }

    pub fn get_area(&mut self) -> usize {
        let mut y = self.min_y;
        let mut current_vertical_lines: Vec<isize> = Vec::new();
        let mut corners_seen: Vec<CornerPosition> = Vec::new();
        let mut area: usize = 0;
        while let Some(new_corner) = self.corners.pop() {
            let Coord { y: new_y, x: new_x } = new_corner.position;
            let corner = new_corner.corner;
            if new_y != y {
                // manually calculate area at y
                let line_area = self.get_line_area(&current_vertical_lines, &corners_seen);
                area += line_area;

                // add to total area the area bounded by vertical lines * height diff
                let cornerless_area = self.get_cornerless_line_area(&current_vertical_lines)
                    * ((new_y - y - 1) as usize);
                area += cornerless_area;

                y = new_y;
                corners_seen.clear();
            }
            if corner.faces(Direction::North) {
                current_vertical_lines.remove(
                    current_vertical_lines
                        .iter()
                        .position(|&x| x == new_x)
                        .unwrap(),
                );
            } else if corner.faces(Direction::South) {
                let insert_pos = current_vertical_lines
                    .binary_search(&new_x)
                    .unwrap_or_else(|p| p);
                current_vertical_lines.insert(insert_pos, new_x);
            }
            corners_seen.push(new_corner);
        }
        area += self.get_line_area(&current_vertical_lines, &corners_seen);

        area
    }
}

fn part2(contents: String) -> usize {
    let mut big_lagoon = BigLagoon::from_str(contents);
    big_lagoon.get_area()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};
    struct Sample {
        pub input_file: &'static str,
        pub part_num: u8,
        pub expected_out: usize,
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
            expected_out: 952408144115,
        }
        .run()
    }
}
