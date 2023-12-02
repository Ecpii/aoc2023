use aoc2023::utils::read_input_file;
use std::cmp::max;
struct CubeCount {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl CubeCount {
    pub fn from_str(s: &str) -> CubeCount {
        let mut res = CubeCount {
            red: 0,
            green: 0,
            blue: 0,
        };
        for cube_color in s.split(',') {
            let mut cube_iter = cube_color.split(' ').skip(1);
            let quantity = cube_iter
                .next()
                .expect("quantity should exist")
                .parse()
                .unwrap();
            match cube_iter.next().unwrap() {
                "red" => res.red = quantity,
                "green" => res.green = quantity,
                "blue" => res.blue = quantity,
                _ => panic!(),
            };
        }
        res
    }

    pub fn merged_with(&self, other: &CubeCount) -> CubeCount {
        CubeCount {
            red: (max(self.red, other.red)),
            green: (max(self.green, other.green)),
            blue: (max(self.blue, other.blue)),
        }
    }

    pub fn get_power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 2207
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2); // 62241
}

fn part1(contents: String) -> u32 {
    let games = contents.split('\n').take_while(|x| !x.is_empty());
    let mut result = 0;
    for game in games {
        let colon_index = game.chars().position(|x| x == ':').unwrap();
        let game_num: u32 = game[5..colon_index].parse().unwrap();
        if game[colon_index + 1..].split(';').all(|game_round| {
            for cube_set in game_round.split(',') {
                let mut string_iter = cube_set.split(' ').skip(1);
                let quantity: usize = string_iter
                    .next()
                    .expect("quantity should exist")
                    .parse()
                    .unwrap();
                let color = string_iter.next().unwrap();
                if (color == "red" && quantity > 12)
                    || (color == "green" && quantity > 13)
                    || (color == "blue" && quantity > 14)
                {
                    return false;
                }
            }
            true
        }) {
            result += game_num
        }
    }
    result
}

fn part2(contents: String) -> usize {
    let games = contents.split('\n').take_while(|x| !x.is_empty());
    let mut result = 0;
    for game in games {
        let miniminum_set: CubeCount = game.split(':').nth(1).unwrap().split(';').fold(
            CubeCount {
                red: 0,
                green: 0,
                blue: 0,
            },
            |accumulator, game_round| accumulator.merged_with(&CubeCount::from_str(game_round)),
        );
        result += miniminum_set.get_power()
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    #[test]
    fn p1sample01() {
        let contents = read_input_file(file!(), "sample.txt");
        let part_1 = part1(contents);
        assert_eq!(part_1, 8);
    }
    #[test]
    fn p2sample01() {
        let contents = read_input_file(file!(), "sample.txt");
        let part_2 = part2(contents);
        assert_eq!(part_2, 2286);
    }
}
