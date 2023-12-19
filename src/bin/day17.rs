use colored::{ColoredString, Colorize};
use std::{cmp::Reverse, collections::BinaryHeap, time::Instant};

use aoc2023::utils::{pretty_print, read_2d_map, read_2d_map_to_u8, read_input_file};
use array2d::Array2D;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    pub y: usize,
    pub x: usize,
}

impl Coord {
    pub fn cardinal_neighbors(&self) -> [Coord; 4] {
        [
            Coord {
                y: self.y.wrapping_sub(1),
                x: self.x,
            },
            Coord {
                y: self.y,
                x: self.x.wrapping_sub(1),
            },
            Coord {
                y: self.y + 1,
                x: self.x,
            },
            Coord {
                y: self.y,
                x: self.x + 1,
            },
        ]
    }

    pub fn direction_to(&self, other: &Coord) -> Direction {
        if other.y < self.y {
            Direction::North
        } else if other.x < self.x {
            Direction::West
        } else if other.y > self.y {
            Direction::South
        } else if other.x > self.x {
            Direction::East
        } else {
            panic!("direction_to called with identical coords!")
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl From<Direction> for char {
    fn from(val: Direction) -> Self {
        match val {
            Direction::North => '^',
            Direction::West => '<',
            Direction::South => 'v',
            Direction::East => '>',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DijkstraEntry {
    pub min_heat_loss: usize,
    pub seen: bool,
    pub source: Option<Coord>,
}

impl DijkstraEntry {
    pub fn new() -> DijkstraEntry {
        DijkstraEntry {
            min_heat_loss: usize::MAX,
            seen: false,
            source: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DistPair {
    pub min_heat_loss: usize,
    pub coord: Coord,
}

// impl Ord for DistPair {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.distance.cmp(&other.distance)
//     }
// }

fn find_restricted_direction(
    dijkstra_map: &Array2D<DijkstraEntry>,
    coord: Coord,
) -> Option<Direction> {
    let mut seen_directions: Vec<Direction> = Vec::with_capacity(3);
    let mut current_coord = coord;
    for _ in 0..3 {
        let DijkstraEntry { source, .. } = dijkstra_map[(current_coord.y, current_coord.x)];

        seen_directions.push(source?.direction_to(&current_coord));
        current_coord = source.unwrap();
    }
    if let Ok(res) = seen_directions.iter().all_equal_value() {
        Some(*res)
    } else {
        None
    }
}

fn highlight_path(dijkstra_map: &Array2D<DijkstraEntry>, map: &Array2D<char>) {
    let width = dijkstra_map.num_columns();
    let height = dijkstra_map.num_rows();
    let mut output_map: Array2D<ColoredString> = Array2D::from_iter_row_major(
        map.elements_row_major_iter()
            .map(|x| x.to_string().normal()),
        height,
        width,
    )
    .unwrap();
    let mut current_coord = Coord {
        y: height - 1,
        x: width - 1,
    };
    let start = Coord { y: 0, x: 0 };
    while current_coord != start {
        let DijkstraEntry { source, .. } = dijkstra_map[(current_coord.y, current_coord.x)];
        let Some(prev_coord) = source else { return };
        let incoming_direction: char = prev_coord.direction_to(&current_coord).into();
        output_map[(prev_coord.y, prev_coord.x)] = incoming_direction.to_string().bold().green();

        current_coord = prev_coord;
    }
    pretty_print(&output_map);
}

fn part1(contents: String) -> isize {
    let map = read_2d_map_to_u8(contents.clone());
    let width = map.num_columns();
    let height = map.num_rows();
    let mut dijkstra_map = Array2D::filled_with(DijkstraEntry::new(), height, width);
    dijkstra_map[(0, 0)].min_heat_loss = 0;
    let mut closest_cities = BinaryHeap::new();
    closest_cities.push(Reverse(DistPair {
        min_heat_loss: 0,
        coord: Coord { x: 0, y: 0 },
    }));

    while !closest_cities.is_empty() {
        let Reverse(DistPair {
            min_heat_loss,
            coord,
        }) = closest_cities.pop().unwrap();

        if min_heat_loss == usize::MAX {
            panic!("how?")
        }

        let dijkstra_entry = &mut dijkstra_map[(coord.y, coord.x)];
        if dijkstra_entry.seen {
            continue;
        }
        dijkstra_entry.seen = true;

        let restricted_direction = find_restricted_direction(&dijkstra_map, coord);
        for neighbor_coord in coord.cardinal_neighbors() {
            let Some(heat_loss) = map.get(neighbor_coord.y, neighbor_coord.x) else {
                continue;
            };
            if restricted_direction.is_some_and(|x| x == coord.direction_to(&neighbor_coord)) {
                continue;
            }

            let neighbor = &mut dijkstra_map[(neighbor_coord.y, neighbor_coord.x)];
            let new_heat_loss = min_heat_loss + (*heat_loss as usize);
            if new_heat_loss < neighbor.min_heat_loss {
                neighbor.min_heat_loss = new_heat_loss;
                neighbor.source = Some(coord);
                closest_cities.push(Reverse(DistPair {
                    min_heat_loss: new_heat_loss,
                    coord: neighbor_coord,
                }))
            }
        }
    }

    let debug_map = read_2d_map(contents);
    highlight_path(&dijkstra_map, &debug_map);

    dijkstra_map[(height - 1, width - 1)].min_heat_loss as isize
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
            expected_out: 102,
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
