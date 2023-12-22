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

    fn get_attribute_bounds(&self, attribute: char) -> impl Iterator<Item = usize> + '_ {
        self.conditions
            .iter()
            .filter(move |cond| cond.attribute == attribute && cond.value != 0)
            .map(|cond| cond.value)
    }
}

static ATTR_NAMES: [char; 4] = ['x', 'm', 'a', 's'];

struct ElfSortingSystem {
    workflows: HashMap<String, Workflow>,
    attribute_bounds: HashMap<char, Vec<usize>>,
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
        let attribute_bounds: HashMap<char, Vec<usize>> = HashMap::with_capacity(4);
        ElfSortingSystem {
            workflows,
            attribute_bounds,
        }
    }

    pub fn check_part(&self, part: &MachinePart) -> bool {
        let mut current_workflow_name = "in";
        while current_workflow_name != "R" && current_workflow_name != "A" {
            let current_workflow = self.workflows.get(current_workflow_name).unwrap();
            current_workflow_name = current_workflow.process(part);
        }
        current_workflow_name == "A"
    }

    fn get_configuration_count(&self, partial_part: &MachinePart) -> usize {
        println!("{:?}", partial_part.attribute_values[0]);
        let Some(attribute_index) = partial_part
            .attribute_values
            .iter()
            .position(|&value| value == 0)
        else {
            return if self.check_part(partial_part) { 1 } else { 0 };
        };
        let mut count = 0;
        let attribute_name = ATTR_NAMES[attribute_index];
        let mut last_attribute_value = 0;
        for attribute_bound in self.attribute_bounds.get(&attribute_name).unwrap() {
            let mut new_machine_part = *partial_part;
            new_machine_part.attribute_values[attribute_index] = *attribute_bound;
            count += self.get_configuration_count(&new_machine_part);

            let inner_attribute_value = attribute_bound - 1;
            if inner_attribute_value != last_attribute_value {
                let interval_width = inner_attribute_value - last_attribute_value;
                new_machine_part.attribute_values[attribute_index] = inner_attribute_value;
                count += self.get_configuration_count(&new_machine_part) * interval_width;
            }
            last_attribute_value = *attribute_bound;
        }

        count
    }

    pub fn merge_count(&mut self) -> usize {
        let workflow_vec: Vec<_> = self.workflows.values().collect();
        // dbg!(&workflow_vec);
        for attribute_name in ATTR_NAMES {
            let mut bounds: Vec<_> = workflow_vec
                .iter()
                .flat_map(|wkflw| wkflw.get_attribute_bounds(attribute_name))
                .filter(|&x| x != 0)
                .collect();
            bounds.sort_unstable();
            bounds.push(4000);
            bounds.dedup_by_key(|x| *x);
            self.attribute_bounds.insert(attribute_name, bounds);
        }

        dbg!(&self.attribute_bounds);
        self.get_configuration_count(&MachinePart {
            attribute_values: [0, 0, 0, 0],
        })
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
    let mut system = ElfSortingSystem::from_str(state_descriptions);
    system.merge_count()
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
