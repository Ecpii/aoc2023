use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1);
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn part1(contents: String) -> usize {
    contents.split('\n').for_each(|_| ());
    0
}

fn part2(contents: String) -> usize {
    contents.split('\n').for_each(|_| ());
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    const P1SAMPLE01_ANSWER: usize = 0;
    const P2SAMPLE01_ANSWER: usize = 0;

    #[test]
    fn p1sample01() {
        let contents = read_input_file(file!(), "sample.txt");
        println!("{:?}", contents);
        let res = part1(contents);
        assert_eq!(res, P1SAMPLE01_ANSWER);
    }
    #[test]
    fn p2sample01() {
        let contents = read_input_file(file!(), "sample.txt");
        println!("{:?}", contents);
        let res = part2(contents);
        assert_eq!(res, P2SAMPLE01_ANSWER);
    }
}
