use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1);
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn part1(contents: String) -> isize {
    let _lines = contents.split('\n').take_while(|x| !x.is_empty());
    0
}

fn part2(contents: String) -> isize {
    let _lines = contents.split('\n').take_while(|x| !x.is_empty());
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    const SAMPLE_ANSWERS: [(isize, isize); 1] = [(0, 0)];

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
}
