use std::collections::HashSet;

use aoc2023::utils::read_input_file;
use array2d::Array2D;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 6875, actually first try, even on the tests wtf i love rust
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
    pub fn west(&self) -> Option<Coord> {
        let new_x = self.x.checked_sub(1);
        new_x.map(|x| Coord { x, y: self.y })
    }
    pub fn north(&self) -> Option<Coord> {
        let new_y = self.y.checked_sub(1);
        new_y.map(|y| Coord { x: self.x, y })
    }
    pub fn east(&self) -> Option<Coord> {
        let new_x = self.x.checked_add(1);
        new_x.map(|x| Coord { x, y: self.y })
    }
    pub fn south(&self) -> Option<Coord> {
        let new_y = self.y.checked_add(1);
        new_y.map(|y| Coord { x: self.x, y })
    }
}

struct PipeMap {
    pub raw_map: Array2D<char>,
    pipe_coords: HashSet<Coord>,
}
impl PipeMap {
    pub fn new(lines: Vec<&str>) -> PipeMap {
        let width = lines[0].len();
        let height = lines.len();
        let flat_iter = lines.iter().flat_map(|x| x.chars());
        PipeMap {
            raw_map: Array2D::from_iter_row_major(flat_iter, height, width).unwrap(),
            pipe_coords: HashSet::new(),
        }
    }

    fn get_starting_position(&self) -> Coord {
        for (y, mut row) in self.raw_map.rows_iter().enumerate() {
            if let Some(x) = row.position(|x| *x == 'S') {
                return Coord { x, y };
            }
        }
        unreachable!()
    }

    fn get(&self, pos: &Coord) -> Option<char> {
        self.raw_map.get(pos.y, pos.x).copied()
    }

    fn get_next_positions(&self, pos: &Coord) -> Option<(Coord, Coord)> {
        let current_pipe = self.get(pos).unwrap();
        match current_pipe {
            '|' => Some((pos.north().unwrap(), pos.south().unwrap())),
            '-' => Some((pos.west().unwrap(), pos.east().unwrap())),
            '7' => Some((pos.west().unwrap(), pos.south().unwrap())),
            'L' => Some((pos.east().unwrap(), pos.north().unwrap())),
            'J' => Some((pos.west().unwrap(), pos.north().unwrap())),
            'F' => Some((pos.east().unwrap(), pos.south().unwrap())),
            'S' => {
                let neighbors = vec![pos.west(), pos.east(), pos.north(), pos.south()];
                let mut res: Vec<Coord> = Vec::with_capacity(2);
                for test_coord in neighbors.into_iter().flatten() {
                    if let Some((first, second)) = self.get_next_positions(&test_coord) {
                        if first == *pos || second == *pos {
                            res.push(test_coord)
                        }
                    }
                }
                assert_eq!(res.len(), 2);
                Some((res[0], res[1]))
            }
            _ => None,
        }
    }

    fn get_next_position(&self, target: &Coord, previous: &Coord) -> Option<Coord> {
        let possible_positions = self.get_next_positions(target);
        possible_positions.map(|(first, second)| if first == *previous { second } else { first })
    }

    pub fn get_farthest_distance_on_loop(&mut self) -> isize {
        let starting_position = self.get_starting_position();
        self.pipe_coords.insert(starting_position);
        let next_positions = self.get_next_positions(&starting_position);
        let (mut left_head, mut right_head) =
            next_positions.expect("Starting position should be connected to pipes");
        self.pipe_coords.insert(left_head);
        self.pipe_coords.insert(right_head);
        let mut slow_left_head = starting_position;
        let mut slow_right_head = starting_position;
        let mut move_counter = 1;
        loop {
            move_counter += 1;
            let mut new_position = self
                .get_next_position(&left_head, &slow_left_head)
                .expect("left head should always be on pipes");
            self.pipe_coords.insert(new_position);
            if new_position == right_head {
                return move_counter;
            } else {
                slow_left_head = left_head;
                left_head = new_position;
            }
            new_position = self
                .get_next_position(&right_head, &slow_right_head)
                .expect("heads should always be on pipes");
            self.pipe_coords.insert(new_position);
            if new_position == left_head {
                return move_counter;
            } else {
                slow_right_head = right_head;
                right_head = new_position;
            }
        }
    }

    pub fn rewrite_start_pos(&mut self) {
        let start_pos = self.get_starting_position();
        let neighbors = self.get_next_positions(&start_pos).unwrap();
        let new_pipe_char = if Some(neighbors.0) == start_pos.west()
            && Some(neighbors.1) == start_pos.east()
        {
            '-'
        } else if Some(neighbors.0) == start_pos.west() && Some(neighbors.1) == start_pos.north() {
            'J'
        } else if Some(neighbors.0) == start_pos.west() && Some(neighbors.1) == start_pos.south() {
            '7'
        } else if Some(neighbors.0) == start_pos.east() && Some(neighbors.1) == start_pos.north() {
            'L'
        } else if Some(neighbors.0) == start_pos.east() && Some(neighbors.1) == start_pos.south() {
            'F'
        } else if Some(neighbors.0) == start_pos.north() && Some(neighbors.1) == start_pos.south() {
            '|'
        } else {
            panic!("Starting point neighbors don't correspond to valid pipe!")
        };

        self.raw_map
            .set(start_pos.y, start_pos.x, new_pipe_char)
            .expect("Failed to rewrite start position.");
    }

    pub fn check_if_enclosed(&self, pos: &Coord) -> bool {
        if self.pipe_coords.contains(pos) {
            return false;
        }
        self.check_west(pos)
        // self.check_west(pos) && self.check_north(pos)
    }

    pub fn check_west(&self, pos: &Coord) -> bool {
        let mut num_borders_seen = 0;
        let mut test_pos = pos.west();
        let mut last_seen_corner: Option<char> = None;
        while test_pos.is_some() {
            let target_coord = test_pos.unwrap();
            if self.pipe_coords.contains(&target_coord) {
                let pipe_char = self.get(&target_coord).unwrap();
                match pipe_char {
                    '-' => (),
                    'L' | 'F' => {
                        if let Some(prev_corner) = last_seen_corner {
                            if prev_corner == '7' && pipe_char == 'L'
                                || prev_corner == 'J' && pipe_char == 'F'
                            {
                                num_borders_seen += 1
                            }
                            last_seen_corner = None;
                        }
                    }
                    '7' | 'J' => {
                        last_seen_corner = Some(pipe_char);
                    }
                    '|' => {
                        num_borders_seen += 1;
                        last_seen_corner = None;
                    }
                    _ => unreachable!(),
                };
            }

            test_pos = test_pos.unwrap().west();
        }
        num_borders_seen & 1 == 1
    }

    pub fn get_enclosed_count(&self) -> isize {
        let mut res = 0;
        let width = self.raw_map.num_columns();
        let height = self.raw_map.num_rows();
        for y in 0..height {
            for x in 0..width {
                if self.check_if_enclosed(&Coord { x, y }) {
                    res += 1
                }
            }
        }
        res
    }
}

fn part1(contents: String) -> isize {
    let lines = contents.split('\n').take_while(|x| !x.is_empty()).collect();
    let mut map = PipeMap::new(lines);
    map.get_farthest_distance_on_loop()
}

fn part2(contents: String) -> isize {
    let lines = contents.split('\n').take_while(|x| !x.is_empty()).collect();
    let mut map = PipeMap::new(lines);
    map.get_farthest_distance_on_loop();
    map.rewrite_start_pos();
    map.get_enclosed_count()
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
    fn square_loop() {
        Sample {
            input_file: "sample.txt",
            part_num: 1,
            expected_out: 4,
        }
        .run()
    }

    #[test]
    fn slightly_complicated() {
        Sample {
            input_file: "sample2.txt",
            part_num: 1,
            expected_out: 8,
        }
        .run()
    }

    #[test]
    fn fake_enclosure() {
        Sample {
            input_file: "sample3.txt",
            part_num: 2,
            expected_out: 4,
        }
        .run()
    }

    #[test]
    fn tight_fake_enclosure() {
        Sample {
            input_file: "sample4.txt",
            part_num: 2,
            expected_out: 4,
        }
        .run()
    }

    #[test]
    fn more_complicated_enclosure() {
        Sample {
            input_file: "sample5.txt",
            part_num: 2,
            expected_out: 8,
        }
        .run()
    }

    #[test]
    fn junk_pipe_enclosure() {
        Sample {
            input_file: "sample6.txt",
            part_num: 2,
            expected_out: 10,
        }
        .run()
    }
}
