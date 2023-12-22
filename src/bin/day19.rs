use std::{collections::HashMap, time::Instant};

use aoc2023::utils::read_input_file;

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let start = Instant::now();
    let part1 = part1(contents);
    let duration = start.elapsed();
    println!("part 1: {}", part1); // 418498
    println!("part 1 took {:?}", duration); // 2.621 ms
    let contents = read_input_file(file!(), "input.txt");
    let start2 = Instant::now();
    let part2 = part2(contents);
    let duration2 = start2.elapsed();
    println!("part 2: {}", part2);
    println!("part 2 took {:?}", duration2);
}

struct MachinePart {
    attribute_values: [usize; 4],
}
impl MachinePart {
    pub fn from_str(description: &str) -> Self {
        let mut value_iter = description[1..description.len() - 1]
            .split(',')
            .map(|x| x[2..].parse::<usize>().unwrap());

        MachinePart {
            attribute_values: [
                value_iter.next().unwrap(),
                value_iter.next().unwrap(),
                value_iter.next().unwrap(),
                value_iter.next().unwrap(),
            ],
        }
    }

    pub fn get_attribute(&self, attribute: char) -> usize {
        match attribute {
            'x' => self.attribute_values[0],
            'm' => self.attribute_values[1],
            'a' => self.attribute_values[2],
            's' => self.attribute_values[3],
            _ => panic!("get attribute called with illegal char"),
        }
    }

    pub fn get_score(&self) -> usize {
        self.attribute_values.iter().sum()
    }
}

struct Condition {
    attribute: char,
    operation: char,
    value: usize,
    destination: String,
}

impl Condition {
    pub fn from_str(description: &str) -> Self {
        if let Some(operation_index) = description.find(|x| x == '<' || x == '>') {
            let colon_index = description.find(':').unwrap();
            let attribute = description.chars().next().unwrap();
            let value: usize = description[operation_index + 1..colon_index]
                .parse()
                .unwrap();
            let destination = &description[colon_index + 1..];
            Condition {
                attribute,
                operation: description.chars().nth(operation_index).unwrap(),
                value,
                destination: destination.to_owned(),
            }
        } else {
            Condition {
                attribute: 'x',
                operation: '>',
                value: 0,
                destination: description.to_owned(),
            }
        }
    }

    fn check(&self, part: &MachinePart) -> Option<&str> {
        let part_value = part.get_attribute(self.attribute);
        if self.operation == '>' && part_value > self.value
            || self.operation == '<' && part_value < self.value
        {
            return Some(&self.destination);
        }
        None
    }
}

struct Workflow {
    conditions: Vec<Condition>,
}

impl Workflow {
    pub fn from_str(description: &str) -> Self {
        let conditions: Vec<Condition> = description.split(',').map(Condition::from_str).collect();
        Workflow { conditions }
    }

    fn process(&self, part: &MachinePart) -> &str {
        for condition in self.conditions.iter() {
            if let Some(destination) = condition.check(part) {
                return destination;
            }
        }
        unreachable!()
    }
}

struct ElfSortingSystem {
    workflows: HashMap<String, Workflow>,
}

impl ElfSortingSystem {
    pub fn from_str(state_lines: &str) -> Self {
        let mut workflows: HashMap<String, Workflow> =
            HashMap::with_capacity(state_lines.chars().filter(|&x| x == '\n').count());
        for line in state_lines.split('\n').take_while(|x| !x.is_empty()) {
            let brace_position = line.chars().position(|x| x == '{').unwrap();
            let key = &line[0..brace_position];
            let value = Workflow::from_str(&line[brace_position + 1..line.len() - 1]);
            workflows.insert(key.to_owned(), value);
        }
        ElfSortingSystem { workflows }
    }

    pub fn check_part(&self, part: &MachinePart) -> bool {
        let mut current_workflow_name = "in";
        while current_workflow_name != "R" && current_workflow_name != "A" {
            let current_workflow = self.workflows.get(current_workflow_name).unwrap();
            current_workflow_name = current_workflow.process(part);
        }
        current_workflow_name == "A"
    }
}

fn part1(contents: String) -> isize {
    let mut file_iter = contents.split("\n\n");
    let state_descriptions = file_iter.next().unwrap();
    let items = file_iter.next().unwrap();
    let system = ElfSortingSystem::from_str(state_descriptions);
    let mut total_score = 0;
    for item in items.split('\n').take_while(|x| !x.is_empty()) {
        let part = MachinePart::from_str(item);
        if system.check_part(&part) {
            total_score += part.get_score();
        }
    }
    total_score as isize
}

fn part2(_contents: String) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{part1, part2, read_input_file};
    struct Sample {
        pub input_file: &'static str,
        pub part_num: u8,
        pub expected_out: isize,
    }
    impl Sample {
        pub fn run(&self) {
            let contents = read_input_file(file!(), self.input_file);
            let start = Instant::now();
            let res = if self.part_num == 1 {
                part1(contents)
            } else {
                part2(contents)
            };
            let duration = start.elapsed();
            println!("test took {:?}", duration);
            assert_eq!(res, self.expected_out);
        }
    }

    #[test]
    fn first_sample() {
        Sample {
            input_file: "sample.txt",
            part_num: 1,
            expected_out: 19114,
        }
        .run()
    }

    #[test]
    fn first_sample_part_two() {
        Sample {
            input_file: "sample.txt",
            part_num: 2,
            expected_out: 0,
        }
        .run()
    }
}
