use std::{
    fs,
    path::{Path, PathBuf},
};

fn get_input_dir() -> PathBuf {
    let current_day: &str = Path::new(file!()).file_stem().unwrap().to_str().unwrap();

    let input_dir: PathBuf = Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("inputs")
        .join(current_day);
    input_dir
}

struct CubeCount {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    let filename = get_input_dir().join("input.txt");
    let contents = fs::read_to_string(filename).expect("opening file failed");
    let (part1, part2) = solve(contents);
    println!("{}", part1)
}

fn solve(contents: String) -> (u32, u32) {
    let games = contents.split('\n').take_while(|x| !x.is_empty());
    let mut part1 = 0;
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
            part1 += game_num
        }
        // game[colon_index + 1..].split([',', ';']).fold(
        //     CubeCount {
        //         red: 0,
        //         green: 0,
        //         blue: 0,
        //     },
        //     |CubeCount { red, green, blue }, string| {
        //         let mut string_iter = string.split(' ').skip(1);
        //         let quantity: usize = string_iter
        //             .next()
        //             .expect("quantity should exist")
        //             .parse()
        //             .unwrap();
        //         match string_iter.next() {
        //             Some("red") => CubeCount {
        //                 red: red + quantity,
        //                 green,
        //                 blue,
        //             },
        //             Some("green") => CubeCount {
        //                 green: green + quantity,
        //                 red,
        //                 blue,
        //             },
        //             Some("blue") => CubeCount {
        //                 blue: blue + quantity,
        //                 green,
        //                 red,
        //             },
        //             _ => panic!("second word should be one of red, green, blue"),
        //         }
        //     },
        // );
    }
    (part1, 0)
}

#[cfg(test)]
mod tests {
    use crate::{get_input_dir, solve};
    use std::fs;

    #[test]
    fn sample_01() {
        let filename = get_input_dir().join("sample.txt");
        let contents = fs::read_to_string(filename).expect("opening file failed");
        println!("{:?}", contents);
        let (part_1, part_2) = solve(contents);
        assert_eq!(part_1, 8);
    }
}
