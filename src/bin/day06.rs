use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 449820
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn get_winning_ways(time: usize, distance: usize) -> usize {
    println!("{time}, {distance}");
    let determinant: f64 = (time * time - 4 * distance) as f64;
    let upper_bound = ((time as f64 + determinant.sqrt()) / 2_f64).floor() as usize;
    let lower_bound = ((time as f64 - determinant.sqrt()) / 2_f64).ceil() as usize;
    let mut winning_ways = upper_bound - lower_bound + 1;

    // check if bounds are exclusive
    let upper_bound_result = upper_bound * (time - upper_bound);
    let lower_bound_result = lower_bound * (time - lower_bound);
    if upper_bound_result <= distance {
        winning_ways -= 1;
    }
    if lower_bound_result <= distance {
        winning_ways -= 1;
    }

    println!("bounds: {:?}, {:?}", lower_bound, upper_bound);
    println!("{:?}", winning_ways);
    winning_ways
}

// y = (n - x) * x
// -y = (x - n) * x
// x^2 - nx + y = 0
// x = (n +- sqrt(n^2 - 4y))/2
fn part1(contents: String) -> usize {
    let mut file_lines = contents.split('\n').take_while(|x| !x.is_empty());
    let times = file_lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());
    let distances = file_lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());

    let mut res = 1;
    for (time, distance) in times.zip(distances) {
        res *= get_winning_ways(time, distance);
    }

    res
}

fn part2(contents: String) -> usize {
    let mut file_lines = contents.split('\n').take_while(|x| !x.is_empty());
    let mut time_string = String::from(file_lines.next().unwrap().split(':').nth(1).unwrap());
    time_string.retain(|x| !x.is_whitespace());
    let time = time_string.parse::<usize>().unwrap();
    let mut distance_string = String::from(file_lines.next().unwrap().split(':').nth(1).unwrap());
    distance_string.retain(|x| !x.is_whitespace());
    let distance = distance_string.parse::<usize>().unwrap();

    get_winning_ways(time, distance)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    const P1SAMPLE01_ANSWER: usize = 288;
    const P2SAMPLE01_ANSWER: usize = 71503;

    #[test]
    fn p1sample01() {
        let contents = read_input_file(file!(), "sample.txt");
        let res = part1(contents);
        assert_eq!(res, P1SAMPLE01_ANSWER);
    }
    #[test]
    fn p2sample01() {
        let contents = read_input_file(file!(), "sample.txt");
        let res = part2(contents);
        assert_eq!(res, P2SAMPLE01_ANSWER);
    }
}
