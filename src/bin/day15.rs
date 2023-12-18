use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 506869
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2)
}

fn hash(input: &str) -> u8 {
    let mut res: usize = 0;
    for char in input.chars() {
        res += char as usize;
        res *= 17;
        res &= 255;
    }
    res as u8
}
fn part1(contents: String) -> isize {
    let clauses = contents.trim_end().split(',');
    let mut res: isize = 0;
    for clause in clauses {
        res += hash(clause) as isize;
    }
    res
}

fn part2(contents: String) -> isize {
    let clauses = contents.trim_end().split(',');
    let num_boxes = 256;
    let mut boxes: Vec<Vec<(&str, u8)>> = Vec::with_capacity(num_boxes);
    for _ in 0..num_boxes {
        boxes.push(Vec::new());
    }

    for clause in clauses {
        let operation = if clause.contains('=') { '=' } else { '-' };
        let end_of_key = if operation == '=' {
            clause.find('=').unwrap()
        } else {
            clause.find('-').unwrap()
        };

        let label = &clause[0..end_of_key];
        let key = hash(label) as usize;
        if operation == '=' {
            let new_focal_length: u8 = clause[end_of_key + 1..].parse().unwrap();
            if let Some(existing_lens_index) = boxes[key]
                .iter()
                .position(|(other_label, _)| *other_label == label)
            {
                boxes[key][existing_lens_index] = (label, new_focal_length);
            } else {
                boxes[key].push((label, new_focal_length));
            }
        } else if operation == '-' {
            if let Some(remove_index) = boxes[key]
                .iter()
                .position(|(other_label, _)| *other_label == label)
            {
                boxes[key as usize].remove(remove_index);
            }
        }
    }

    let mut res = 0;

    for (box_number, box_lenses) in boxes.iter().enumerate() {
        for (lens_position, (_, focal_length)) in box_lenses.iter().enumerate() {
            res += (box_number + 1) * (lens_position + 1) * (*focal_length as usize);
        }
    }
    res as isize
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
            expected_out: 1320,
        }
        .run()
    }

    #[test]
    fn first_sample_part_two() {
        Sample {
            input_file: "sample.txt",
            part_num: 2,
            expected_out: 145,
        }
        .run()
    }
}
