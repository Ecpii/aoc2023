use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc2023::utils::{read_2d_map, read_input_file};
use array2d::Array2D;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 8249
    let contents = read_input_file(file!(), "input.txt");
    let start2 = Instant::now();
    let part2 = part2(contents);
    let duration2 = start2.elapsed();
    println!("Time elapsed for part 2: {:?}", duration2);
    println!("part 2: {}", part2)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn is_vertical(&self) -> bool {
        *self == Direction::North || *self == Direction::South
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Beam {
    position: Coord,
    direction: Direction,
}

impl Beam {
    pub fn shift(&mut self) {
        match self.direction {
            Direction::North => {
                self.position = Coord::new(self.position.x, self.position.y.wrapping_sub(1))
            }
            Direction::West => {
                self.position = Coord::new(self.position.x.wrapping_sub(1), self.position.y)
            }
            Direction::South => self.position = Coord::new(self.position.x, self.position.y + 1),
            Direction::East => self.position = Coord::new(self.position.x + 1, self.position.y),
        };
    }

    pub fn reflect(&mut self, mirror: char) {
        if mirror == '/' {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                Direction::South => Direction::West,
                Direction::East => Direction::North,
            }
        } else {
            self.direction = match self.direction {
                Direction::North => Direction::West,
                Direction::West => Direction::North,
                Direction::South => Direction::East,
                Direction::East => Direction::South,
            }
        }
    }

    pub fn handle_split(&mut self, splitter: char) -> Option<Beam> {
        if (self.direction.is_vertical() && splitter == '|')
            || (!self.direction.is_vertical() && splitter == '-')
        {
            self.shift();
            return None;
        }

        let mut new_beam = *self;
        if splitter == '|' {
            self.direction = Direction::North;
            new_beam.direction = Direction::South;
        } else {
            self.direction = Direction::West;
            new_beam.direction = Direction::East;
        }
        Some(new_beam)
    }
}
fn part1(contents: String) -> isize {
    let map = read_2d_map(contents);
    count_energized_tiles(
        &map,
        Beam {
            position: Coord::new(0, 0),
            direction: Direction::East,
        },
    )
}

fn count_energized_tiles(map: &Array2D<char>, starting_beam: Beam) -> isize {
    let mut energized_tiles: HashMap<Coord, HashSet<Direction>> = HashMap::new();
    let mut beams: Vec<Beam> = vec![starting_beam];
    while !beams.is_empty() {
        let mut future_beams: Vec<Beam> = Vec::new();
        let mut removed_beams: HashSet<Beam> = HashSet::new();

        for beam in beams.iter_mut() {
            let Coord { x, y } = beam.position;
            if energized_tiles
                .get(&beam.position)
                .is_some_and(|directions| directions.contains(&beam.direction))
            {
                removed_beams.insert(*beam);
                continue;
            }

            if let Some(current_space) = map.get(y, x) {
                let position_entry = energized_tiles
                    .entry(beam.position)
                    .or_insert_with(|| HashSet::with_capacity(4));
                position_entry.insert(beam.direction);
                match current_space {
                    '.' => beam.shift(),
                    '/' | '\\' => {
                        beam.reflect(*current_space);
                        beam.shift();
                    }
                    '|' | '-' => {
                        if let Some(split_beam) = beam.handle_split(*current_space) {
                            future_beams.push(split_beam)
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                removed_beams.insert(*beam);
            }
        }

        beams.retain(|x| !removed_beams.contains(x));
        beams.extend(future_beams);
    }

    energized_tiles.len() as isize
}

fn part2(contents: String) -> isize {
    let map = read_2d_map(contents);
    let mut max_energized_tiles = 0;
    for x in 0..map.num_columns() {
        max_energized_tiles = max(
            count_energized_tiles(
                &map,
                Beam {
                    position: Coord { x, y: 0 },
                    direction: Direction::South,
                },
            ),
            max_energized_tiles,
        );
        max_energized_tiles = max(
            count_energized_tiles(
                &map,
                Beam {
                    position: Coord {
                        x,
                        y: map.num_rows() - 1,
                    },
                    direction: Direction::North,
                },
            ),
            max_energized_tiles,
        );
    }
    for y in 0..map.num_rows() {
        max_energized_tiles = max(
            count_energized_tiles(
                &map,
                Beam {
                    position: Coord { x: 0, y },
                    direction: Direction::East,
                },
            ),
            max_energized_tiles,
        );
        max_energized_tiles = max(
            count_energized_tiles(
                &map,
                Beam {
                    position: Coord {
                        x: map.num_columns() - 1,
                        y,
                    },
                    direction: Direction::West,
                },
            ),
            max_energized_tiles,
        );
    }
    max_energized_tiles
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
            expected_out: 46,
        }
        .run()
    }

    #[test]
    fn first_sample_part_two() {
        Sample {
            input_file: "sample.txt",
            part_num: 2,
            expected_out: 51,
        }
        .run()
    }
}
