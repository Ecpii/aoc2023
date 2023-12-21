use colored::Colorize;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, VecDeque},
    time::Instant,
};

use aoc2023::utils::{pretty_print, read_2d_map_to_u8, read_input_file};
use array2d::Array2D;
// this is god awful code

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let start = Instant::now();
    let part1 = part1(contents);
    let duration = start.elapsed();
    println!("part 1: {}", part1); // 1013
    println!("part 1 took {:?}", duration); // 26.82 s
    let contents = read_input_file(file!(), "input.txt");
    let start2 = Instant::now();
    let part2 = part2(contents);
    let duration2 = start2.elapsed();
    println!("part 2: {}", part2); // 1215
    println!("part 2 took {:?}", duration2); // 27.5 s
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    pub y: usize,
    pub x: usize,
}

impl Coord {
    pub fn cardinal_neighbors(&self) -> [Coord; 4] {
        [
            self.shifted(Direction::North, 1),
            self.shifted(Direction::West, 1),
            self.shifted(Direction::South, 1),
            self.shifted(Direction::East, 1),
        ]
    }

    pub fn _cardinal_distant_neighbors(&self, steps: usize) -> [Coord; 4] {
        [
            self.shifted(Direction::North, steps),
            self.shifted(Direction::West, steps),
            self.shifted(Direction::South, steps),
            self.shifted(Direction::East, steps),
        ]
    }

    pub fn cross_neighbors(&self, d: Direction, steps: usize) -> [Coord; 2] {
        match d {
            Direction::North | Direction::South => [
                self.shifted(Direction::West, steps),
                self.shifted(Direction::East, steps),
            ],
            Direction::West | Direction::East => [
                self.shifted(Direction::North, steps),
                self.shifted(Direction::South, steps),
            ],
        }
    }

    pub fn shifted(&self, d: Direction, steps: usize) -> Coord {
        match d {
            Direction::North => Coord {
                y: self.y.wrapping_sub(steps),
                x: self.x,
            },
            Direction::West => Coord {
                y: self.y,
                x: self.x.wrapping_sub(steps),
            },
            Direction::South => Coord {
                y: self.y + steps,
                x: self.x,
            },
            Direction::East => Coord {
                y: self.y,
                x: self.x + steps,
            },
        }
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
impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
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
    pub sources: Vec<Coord>,
}

impl DijkstraEntry {
    pub fn new() -> DijkstraEntry {
        DijkstraEntry {
            min_heat_loss: usize::MAX,
            seen: false,
            sources: Vec::with_capacity(4),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DistPair {
    pub min_heat_loss: usize,
    pub coord: Coord,
}

struct Solver {
    map: Array2D<u8>,
    dijkstra_map: Array2D<DijkstraEntry>,
    closest_cities: BinaryHeap<Reverse<DistPair>>,
}

struct SearchStackFrame {
    position: Coord,
    restrictive_direction: Direction,
    direction_ttl: usize,
    total_loss: usize,
}

#[derive(Hash, Eq, PartialEq)]
struct PartialSearchStackFrame {
    position: Coord,
    restrictive_direction: Direction,
}

impl Solver {
    fn new(map: Array2D<u8>) -> Solver {
        let width = map.num_columns();
        let height = map.num_rows();
        let mut dijkstra_map = Array2D::filled_with(DijkstraEntry::new(), height, width);
        dijkstra_map[(0, 0)].min_heat_loss = 0;
        let mut closest_cities = BinaryHeap::new();
        closest_cities.push(Reverse(DistPair {
            min_heat_loss: 0,
            coord: Coord { x: 0, y: 0 },
        }));
        Solver {
            map,
            dijkstra_map,
            closest_cities,
        }
    }

    fn width(&self) -> usize {
        self.map.num_columns()
    }
    fn height(&self) -> usize {
        self.map.num_rows()
    }

    fn get(&self, c: &Coord) -> Option<&u8> {
        self.map.get(c.y, c.x)
    }
    fn get_dijkstra(&self, c: &Coord) -> Option<&DijkstraEntry> {
        self.dijkstra_map.get(c.y, c.x)
    }
    fn get_dijkstra_mut(&mut self, c: &Coord) -> Option<&mut DijkstraEntry> {
        self.dijkstra_map.get_mut(c.y, c.x)
    }
    fn is_illegal_movement(&self, start: Coord, end: Coord) -> bool {
        let end_direction = start.direction_to(&end);
        let mut current_coord = start;
        for _ in 0..3 {
            let DijkstraEntry { sources, .. } = self.get_dijkstra(&current_coord).unwrap();

            if sources.len() != 1 || sources[0].direction_to(&start) != end_direction {
                return false;
            }
            current_coord = sources[0];
        }
        true
    }

    fn run_dijkstra(&mut self) {
        while !self.closest_cities.is_empty() {
            let Reverse(DistPair {
                min_heat_loss,
                coord,
            }) = self.closest_cities.pop().unwrap();

            let dijkstra_entry = self.get_dijkstra_mut(&coord).unwrap();
            if dijkstra_entry.seen {
                continue;
            }
            dijkstra_entry.seen = true;

            for neighbor_coord in coord.cardinal_neighbors() {
                let Some(&heat_loss) = self.get(&neighbor_coord) else {
                    continue;
                };
                if self.is_illegal_movement(coord, neighbor_coord) {
                    continue;
                }

                let neighbor = self.get_dijkstra_mut(&neighbor_coord).unwrap();
                let new_heat_loss = min_heat_loss + (heat_loss as usize);
                match new_heat_loss.cmp(&neighbor.min_heat_loss) {
                    Ordering::Less => {
                        neighbor.min_heat_loss = new_heat_loss;
                        neighbor.sources.clear();
                        neighbor.sources.push(coord);
                        self.closest_cities.push(Reverse(DistPair {
                            min_heat_loss: new_heat_loss,
                            coord: neighbor_coord,
                        }))
                    }
                    Ordering::Equal => {
                        neighbor.sources.push(coord);
                    }
                    Ordering::Greater => (),
                }
            }
        }
    }

    fn _soft_reset_dijkstra(&mut self) {
        self.dijkstra_map[(0, 0)].min_heat_loss = 0;
        for i in 0..self.width() * self.height() {
            let entry = self.dijkstra_map.get_mut_row_major(i).unwrap();
            entry.seen = false;
        }
        self.closest_cities.push(Reverse(DistPair {
            min_heat_loss: 0,
            coord: Coord { x: 0, y: 0 },
        }));
    }

    fn search_with_bound(&mut self) -> isize {
        let mut upper_bound =
            self.dijkstra_map[(self.height() - 1, self.width() - 1)].min_heat_loss;
        let end_coord = Coord {
            y: self.height() - 1,
            x: self.width() - 1,
        };
        let mut positions_to_scan: VecDeque<SearchStackFrame> = VecDeque::new();
        positions_to_scan.push_back(SearchStackFrame {
            position: Coord { x: 0, y: 0 },
            restrictive_direction: Direction::South,
            direction_ttl: 4,
            total_loss: 0,
        });
        let mut positions_seen: HashMap<PartialSearchStackFrame, (usize, usize)> = HashMap::new();

        while let Some(SearchStackFrame {
            position,
            restrictive_direction,
            direction_ttl,
            total_loss,
        }) = positions_to_scan.pop_front()
        {
            // println!("{}", total_loss);
            for neighbor in position.cardinal_neighbors() {
                let Some(&heat_loss) = self.get(&neighbor) else {
                    continue;
                };
                let new_direction = position.direction_to(&neighbor);
                let new_ttl = if new_direction == restrictive_direction {
                    direction_ttl - 1
                } else {
                    3
                };
                let new_loss = total_loss + (heat_loss as usize);

                if new_direction == restrictive_direction.opposite()
                    || new_ttl == 0
                    || new_loss >= upper_bound
                {
                    continue;
                }

                if neighbor == end_coord {
                    upper_bound = new_loss;
                    break;
                }

                if let Some((past_ttl, past_loss)) = positions_seen.get(&PartialSearchStackFrame {
                    position: neighbor,
                    restrictive_direction: new_direction,
                }) {
                    if new_ttl <= *past_ttl && new_loss >= *past_loss {
                        continue;
                    }
                }

                positions_seen.insert(
                    PartialSearchStackFrame {
                        position: neighbor,
                        restrictive_direction: new_direction,
                    },
                    (new_ttl, new_loss),
                );
                positions_to_scan.push_back(SearchStackFrame {
                    position: neighbor,
                    restrictive_direction: new_direction,
                    direction_ttl: new_ttl,
                    total_loss: new_loss,
                })
            }
        }
        upper_bound as isize
    }
    fn run(&mut self) -> isize {
        self.run_dijkstra();
        // self.highlight_path();
        // println!(
        //     "{}",
        //     self.dijkstra_map[(self.height() - 1, self.width() - 1)].min_heat_loss
        // );
        self.search_with_bound()
    }
    fn _highlight_path(&self) {
        let mut map = Array2D::from_iter_row_major(
            self.map
                .elements_row_major_iter()
                .map(|x| x.to_string().normal()),
            self.height(),
            self.width(),
        )
        .unwrap();
        let mut current_coord = Coord {
            y: self.height() - 1,
            x: self.width() - 1,
        };
        let start = Coord { y: 0, x: 0 };
        while current_coord != start {
            let DijkstraEntry { sources, .. } = self.get_dijkstra(&current_coord).unwrap();
            if sources.is_empty() {
                return;
            }
            let prev_coord = sources[0];
            let incoming_direction: char = prev_coord.direction_to(&current_coord).into();
            map[(prev_coord.y, prev_coord.x)] = incoming_direction.to_string().bold().green();

            current_coord = prev_coord;
        }
        pretty_print(&map);
    }
}

fn part1(contents: String) -> isize {
    let map = read_2d_map_to_u8(contents.clone());
    let mut solver = Solver::new(map);
    solver.run()
    // let debug_map = read_2d_map(contents);
    // highlight_path(&dijkstra_map, &debug_map);

    // dijkstra_map[(height - 1, width - 1)].min_heat_loss as isize
}

struct Part2Solver {
    map: Array2D<u8>,
    positions_to_scan: VecDeque<SearchStackFrame>,
    positions_seen: HashMap<PartialSearchStackFrame, (usize, usize)>,
    upper_bound: usize,
}
impl Part2Solver {
    fn new(map: Array2D<u8>) -> Self {
        let mut positions_to_scan = VecDeque::new();
        positions_to_scan.push_back(SearchStackFrame {
            position: Coord { x: 0, y: 0 },
            restrictive_direction: Direction::West,
            direction_ttl: 6,
            total_loss: 0,
        });
        positions_to_scan.push_back(SearchStackFrame {
            position: Coord { x: 0, y: 0 },
            restrictive_direction: Direction::North,
            direction_ttl: 6,
            total_loss: 0,
        });
        let positions_seen: HashMap<PartialSearchStackFrame, (usize, usize)> = HashMap::new();
        Part2Solver {
            map,
            positions_to_scan,
            positions_seen,
            upper_bound: usize::MAX,
        }
    }

    fn width(&self) -> usize {
        self.map.num_columns()
    }
    fn height(&self) -> usize {
        self.map.num_rows()
    }

    fn end(&self) -> Coord {
        Coord {
            y: self.height() - 1,
            x: self.width() - 1,
        }
    }

    fn get(&self, c: &Coord) -> Option<&u8> {
        self.map.get(c.y, c.x)
    }
    fn straight_loss_between(&self, start: Coord, end: Coord) -> usize {
        let mut loss: usize = 0;
        let travel_direction = start.direction_to(&end);
        if travel_direction == Direction::North || travel_direction == Direction::West {
            for x in end.x..start.x {
                loss += *self.get(&Coord { x, y: start.y }).unwrap() as usize;
            }
            for y in end.y..start.y {
                loss += *self.get(&Coord { x: start.x, y }).unwrap() as usize;
            }
        } else {
            for x in start.x + 1..=end.x {
                loss += *self.get(&Coord { x, y: start.y }).unwrap() as usize;
            }
            for y in start.y + 1..=end.y {
                loss += *self.get(&Coord { x: start.x, y }).unwrap() as usize;
            }
        }
        loss
    }

    pub fn handle_front_neighbor(
        &mut self,
        SearchStackFrame {
            position,
            restrictive_direction,
            direction_ttl,
            total_loss,
        }: &SearchStackFrame,
    ) {
        let front_neighbor = position.shifted(*restrictive_direction, 1);
        let Some(&heat_loss) = self.get(&front_neighbor) else {
            return;
        };
        let new_ttl = direction_ttl - 1;
        let new_loss = total_loss + (heat_loss as usize);
        if new_ttl == 0 || new_loss >= self.upper_bound {
            return;
        }
        if front_neighbor == self.end() {
            self.upper_bound = new_loss;
            // println!("\nEnd found by straight!");
            // dbg!(&position);
            // dbg!(&restrictive_direction);
            // dbg!(&total_loss);
            return;
        }

        if let Some((past_ttl, past_loss)) = self.positions_seen.get(&PartialSearchStackFrame {
            position: front_neighbor,
            restrictive_direction: *restrictive_direction,
        }) {
            if new_ttl <= *past_ttl && new_loss >= *past_loss {
                return;
            }
        }
        self.positions_to_scan.push_back(SearchStackFrame {
            position: front_neighbor,
            restrictive_direction: *restrictive_direction,
            direction_ttl: new_ttl,
            total_loss: new_loss,
        })
    }

    fn handle_side_neighbors(
        &mut self,
        SearchStackFrame {
            position,
            restrictive_direction,
            direction_ttl: _,
            total_loss,
        }: &SearchStackFrame,
    ) {
        let turn_neighbors = position.cross_neighbors(*restrictive_direction, 4);

        for neighbor in turn_neighbors {
            if self.get(&neighbor).is_none() {
                continue;
            };
            let new_direction = position.direction_to(&neighbor);
            let new_ttl = 7;
            let new_loss = total_loss + self.straight_loss_between(*position, neighbor);

            if new_loss >= self.upper_bound {
                continue;
            }

            if neighbor == self.end() {
                // println!("\nEnd found by sidestep!");
                // dbg!(&neighbor);
                // dbg!(&position);
                // dbg!(&restrictive_direction);
                // dbg!(&total_loss);
                self.upper_bound = new_loss;
                return;
            }

            if let Some((past_ttl, past_loss)) = self.positions_seen.get(&PartialSearchStackFrame {
                position: neighbor,
                restrictive_direction: new_direction,
            }) {
                if new_ttl <= *past_ttl && new_loss >= *past_loss {
                    continue;
                }
            }

            self.positions_seen.insert(
                PartialSearchStackFrame {
                    position: neighbor,
                    restrictive_direction: new_direction,
                },
                (new_ttl, new_loss),
            );
            self.positions_to_scan.push_back(SearchStackFrame {
                position: neighbor,
                restrictive_direction: new_direction,
                direction_ttl: new_ttl,
                total_loss: new_loss,
            })
        }
    }

    pub fn run(&mut self) -> isize {
        while let Some(env) = self.positions_to_scan.pop_front() {
            self.handle_front_neighbor(&env);
            // println!"{}", total_loss);
            self.handle_side_neighbors(&env);
        }
        self.upper_bound as isize
    }
}

fn part2(contents: String) -> isize {
    let map = read_2d_map_to_u8(contents.clone());
    let mut solver = Part2Solver::new(map);
    solver.run()
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

    // #[test]
    // fn first_divergence_first_sample() {
    //     Sample {
    //         input_file: "sample2.txt",
    //         part_num: 1,
    //         expected_out: 34,
    //     }
    //     .run()
    // }

    #[test]
    fn first_sample_part_two() {
        Sample {
            input_file: "sample.txt",
            part_num: 2,
            expected_out: 94,
        }
        .run()
    }

    #[test]
    fn third_sample_part_two() {
        Sample {
            input_file: "sample3.txt",
            part_num: 2,
            expected_out: 71,
        }
        .run()
    }
}
