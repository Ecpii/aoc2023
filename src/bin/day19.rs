use std::{
    cmp::{max, min},
    collections::HashMap,
    time::Instant,
};

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

static ATTR_NAMES: [char; 4] = ['x', 'm', 'a', 's'];
#[derive(Copy, Clone)]
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
        let attribute_index = ATTR_NAMES.iter().position(|&x| x == attribute).unwrap();
        self.attribute_values[attribute_index]
    }

    pub fn get_score(&self) -> usize {
        self.attribute_values.iter().sum()
    }
}

#[derive(Copy, Clone)]
struct MachinePartRange {
    attribute_ranges: [(usize, usize); 4],
}

impl MachinePartRange {
    pub fn new() -> Self {
        MachinePartRange {
            attribute_ranges: [(1, 4000); 4],
        }
    }

    pub fn size(&self) -> usize {
        self.attribute_ranges
            .iter()
            .map(|(start, end)| (end + 1).saturating_sub(*start))
            .product()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn split_by(&self, condition: &Condition) -> (Self, Self) {
        let attribute_index = ATTR_NAMES
            .iter()
            .position(|&x| x == condition.attribute)
            .unwrap();
        let (start, end) = self.attribute_ranges[attribute_index];
        if condition.operation == '>' {
            let new_start = max(start, condition.value + 1);
            (
                self.with(attribute_index, (new_start, end)),
                self.with(attribute_index, (start, new_start - 1)),
            )
        } else {
            let new_end = min(end, condition.value - 1);
            (
                self.with(attribute_index, (start, new_end)),
                self.with(attribute_index, (new_end + 1, end)),
            )
        }
    }

    fn with(&self, attribute_index: usize, new_range: (usize, usize)) -> MachinePartRange {
        let mut res = *self;
        res.attribute_ranges[attribute_index] = new_range;
        res
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

    pub fn traverse_count(&self, workflow_name: &str, range: MachinePartRange) -> usize {
        if range.is_empty() || workflow_name == "R" {
            return 0;
        }
        if workflow_name == "A" {
            return range.size();
        }
        let workflow = self.workflows.get(workflow_name).unwrap();
        let mut current_range = range;
        let mut count = 0;
        for condition in workflow.conditions.iter() {
            let (split_range, new_range) = current_range.split_by(condition);
            count += self.traverse_count(&condition.destination, split_range);
            current_range = new_range;
        }
        count
    }
}

fn part1(contents: String) -> usize {
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
    total_score
}

fn part2(contents: String) -> usize {
    let state_descriptions = contents.split("\n\n").next().unwrap();
    let system = ElfSortingSystem::from_str(state_descriptions);
    system.traverse_count("in", MachinePartRange::new())
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{part1, part2, read_input_file};
    struct Sample {
        pub input_file: &'static str,
        pub part_num: u8,
        pub expected_out: usize,
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
            expected_out: 167409079868000,
        }
        .run()
    }
}
