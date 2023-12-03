use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 537832
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn check_slice_symbol(slice: &str, start: usize, stop: usize) -> bool {
    // println!("start: {}, stop: {}", start, stop);
    slice.get(start..stop).unwrap().chars().any(|x| x != '.')
}

fn part1(contents: String) -> usize {
    let mut res = 0;
    let lines: Vec<_> = contents.split('\n').take_while(|x| !x.is_empty()).collect();
    for (line_num, line) in lines.iter().enumerate() {
        // println!("{}", line);
        let mut current_number_str = String::new();
        for (index, char) in line.char_indices() {
            if !char.is_ascii_digit() {
                if !current_number_str.is_empty() {
                    let current_number: usize = current_number_str.parse().unwrap();
                    let number_left_ind = index.saturating_sub(current_number_str.len() + 1);
                    let mut has_neighboring_symbol = false;
                    if let Some(upper_line) = lines.get(line_num.wrapping_sub(1)) {
                        has_neighboring_symbol |=
                            check_slice_symbol(upper_line, number_left_ind, index + 1);
                    }

                    let left_character = line.chars().nth(number_left_ind).unwrap();
                    has_neighboring_symbol |=
                        !left_character.is_ascii_digit() && left_character != '.';
                    has_neighboring_symbol |= char != '.';
                    if let Some(lower_line) = lines.get(line_num + 1) {
                        has_neighboring_symbol |=
                            check_slice_symbol(lower_line, number_left_ind, index + 1);
                    }

                    if has_neighboring_symbol {
                        // println!("recognized {} as a valid number", current_number);
                        res += current_number;
                    }
                    current_number_str = String::new();
                }
                continue;
            }

            current_number_str.push(char);
        }
        if !current_number_str.is_empty() {
            let current_number: usize = current_number_str.parse().unwrap();
            let number_left_ind = line.len().saturating_sub(current_number_str.len() + 1);
            let mut has_neighboring_symbol = false;
            if let Some(upper_line) = lines.get(line_num.wrapping_sub(1)) {
                has_neighboring_symbol |=
                    check_slice_symbol(upper_line, number_left_ind, line.len());
            }

            let left_character = line.chars().nth(number_left_ind).unwrap();
            has_neighboring_symbol |= !left_character.is_ascii_digit() && left_character != '.';
            if let Some(lower_line) = lines.get(line_num + 1) {
                has_neighboring_symbol |=
                    check_slice_symbol(lower_line, number_left_ind, line.len());
            }

            if has_neighboring_symbol {
                // println!("recognized {} as a valid number", current_number);
                res += current_number;
            }
        }
    }
    res
}

fn part2(contents: String) -> usize {
    contents.split('\n').for_each(|_| ());
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    const P1SAMPLE01_ANSWER: usize = 4361;
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
