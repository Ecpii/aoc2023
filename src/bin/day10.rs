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

#[derive(PartialEq, Debug, Clone, Copy)]
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
}
impl PipeMap {
    pub fn new(lines: Vec<&str>) -> PipeMap {
        let width = lines[0].len();
        let height = lines.len();
        let flat_iter = lines.iter().flat_map(|x| x.chars());
        PipeMap {
            raw_map: Array2D::from_iter_row_major(flat_iter, height, width).unwrap(),
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

    pub fn get_farthest_distance_on_loop(&self) -> isize {
        let starting_position = self.get_starting_position();
        let next_positions = self.get_next_positions(&starting_position);
        let (mut left_head, mut right_head) =
            next_positions.expect("Starting position should be connected to pipes");
        let mut slow_left_head = starting_position;
        let mut slow_right_head = starting_position;
        let mut move_counter = 1;
        loop {
            move_counter += 1;
            let mut new_position = self
                .get_next_position(&left_head, &slow_left_head)
                .expect("left head should always be on pipes");
            if new_position == right_head {
                return move_counter;
            } else {
                slow_left_head = left_head;
                left_head = new_position;
            }
            new_position = self
                .get_next_position(&right_head, &slow_right_head)
                .expect("heads should always be on pipes");
            if new_position == left_head {
                return move_counter;
            } else {
                slow_right_head = right_head;
                right_head = new_position;
            }
        }
    }
}

fn part1(contents: String) -> isize {
    let lines = contents.split('\n').take_while(|x| !x.is_empty()).collect();
    let map = PipeMap::new(lines);
    map.get_farthest_distance_on_loop()
}

fn part2(contents: String) -> isize {
    let _lines = contents.split('\n').take_while(|x| !x.is_empty());
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    const SAMPLE_ANSWERS: [(isize, isize); 2] = [(4, 0), (8, 0)];

    #[test]
    fn sample01p1() {
        let contents = read_input_file(file!(), "sample.txt");
        let res = part1(contents);
        assert_eq!(res, SAMPLE_ANSWERS[0].0);
    }
    #[test]
    fn sample01p2() {
        let contents = read_input_file(file!(), "sample.txt");
        let res = part2(contents);
        assert_eq!(res, SAMPLE_ANSWERS[0].1);
    }

    #[test]
    fn sample02p1() {
        let contents = read_input_file(file!(), "sample2.txt");
        let res = part1(contents);
        assert_eq!(res, SAMPLE_ANSWERS[1].0);
    }
    #[test]
    fn sample02p2() {
        let contents = read_input_file(file!(), "sample2.txt");
        let res = part2(contents);
        assert_eq!(res, SAMPLE_ANSWERS[1].1);
    }
}
