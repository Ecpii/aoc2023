use itertools::Itertools;
use std::cmp::min;

use aoc2023::utils::read_input_file;

#[derive(Debug, Clone)]
struct SeedRange {
    pub start: usize,
    pub length: usize,
}

impl SeedRange {
    pub fn contains(&self, test: usize) -> bool {
        test.checked_sub(self.start)
            .map(|x| x <= self.length)
            .unwrap_or(false)
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }

    pub fn partition_by(&self, ranges: &Vec<MapRange>) -> Vec<SeedRange> {
        let mut source_ranges = Vec::new();
        let mut dest_ranges = Vec::new();
        for range in ranges {
            // range: .....|-----|
            // self : ..|-----|...
            if self.contains(range.source_start) {
                let range_end = min(self.end(), range.source_end());
                let length = range_end - range.source_start;
                source_ranges.push(SeedRange {
                    start: range.source_start,
                    length,
                });
                dest_ranges.push(SeedRange {
                    start: range.dest_start,
                    length,
                });
            }
            // range: ..|-----|...
            // self : .....|-----|
            else if self.contains(range.source_end()) {
                let length = range.source_end() - self.start;
                source_ranges.push(SeedRange {
                    start: self.start,
                    length,
                });
                dest_ranges.push(SeedRange {
                    start: range.dest_end() - length,
                    length,
                })
            }
            // range: ..|-----------|
            // self : .....|-----|...
            else if range.source_end() > self.end() && range.source_start < self.start {
                if !dest_ranges.is_empty() {
                    panic!("Overlapping ranges exist?")
                }
                let left_offset = self.start - range.source_start;
                let new_start = range.dest_start + left_offset;
                return vec![SeedRange {
                    start: new_start,
                    length: self.length,
                }];
            }
        }
        if dest_ranges.is_empty() {
            return vec![self.clone()];
        }

        source_ranges.sort_unstable_by(|x, y| x.start.cmp(&y.start));
        let mut left_end = self.start;
        let mut right_end: usize;
        for range in source_ranges.iter() {
            right_end = range.start;
            let gap_length = right_end - left_end;
            if right_end - left_end != 0 {
                dest_ranges.push(SeedRange {
                    start: left_end,
                    length: gap_length,
                });
            }
            left_end = range.end();
        }

        dest_ranges
    }
}

#[derive(Debug)]
struct MapRange {
    pub source_start: usize,
    pub dest_start: usize,
    pub length: usize,
}

impl MapRange {
    pub fn transform(&self, input_num: usize) -> usize {
        if let Some(diff) = input_num.checked_sub(self.source_start) {
            if diff <= self.length {
                return self.dest_start + diff;
            }
        }
        input_num
    }

    pub fn contains(&self, input_num: usize) -> bool {
        if let Some(diff) = input_num.checked_sub(self.source_start) {
            if diff <= self.length {
                return true;
            }
        }
        false
    }

    pub fn source_end(&self) -> usize {
        self.source_start + self.length
    }
    pub fn dest_end(&self) -> usize {
        self.dest_start + self.length
    }
}

fn main() {
    let contents = read_input_file(file!(), "input.txt");
    let part1 = part1(contents);
    println!("part 1: {}", part1); // 836040384
    let contents = read_input_file(file!(), "input.txt");
    let part2 = part2(contents);
    println!("part 2: {}", part2); // 10834440
}

fn part1(contents: String) -> usize {
    let mut file_iter = contents.split("\n\n");
    let seeds_input = file_iter.next().unwrap();
    let mut seeds: Vec<_> = seeds_input
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| vec![x.parse::<usize>().unwrap(); 8])
        // .map(|x| Seed::new(x.parse().unwrap()))
        .collect();

    for i in 1..8 {
        let section = file_iter.next().unwrap();
        let line_reader = section.split('\n').skip(1).take_while(|x| !x.is_empty());
        let mut ranges: Vec<MapRange> = Vec::new();
        for line in line_reader {
            let mut line_numbers = line.split(' ').map(|x| x.parse::<usize>().unwrap());
            let range = MapRange {
                dest_start: line_numbers.next().unwrap(),
                source_start: line_numbers.next().unwrap(),
                length: line_numbers.next().unwrap(),
            };

            ranges.push(range);
        }
        seeds = seeds
            .iter_mut()
            .map(|seed| {
                let mut new_seed = seed.clone();
                new_seed[i] = ranges
                    .iter()
                    .find(|range| range.contains(seed[i - 1]))
                    .map(|range| range.transform(seed[i - 1]))
                    .unwrap_or(seed[i - 1]);
                new_seed
            })
            .collect();
    }

    seeds
        .iter()
        .fold(usize::MAX, |current_min, seed| min(seed[7], current_min))
}

fn part2(contents: String) -> usize {
    let mut file_iter = contents.split("\n\n");
    let seeds_input = file_iter.next().unwrap();
    let mut seeds: Vec<_> = seeds_input
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .chunks(2)
        .into_iter()
        .map(|mut chunks| SeedRange {
            start: chunks.next().unwrap().parse().unwrap(),
            length: chunks.next().unwrap().parse().unwrap(),
        })
        .collect();
    for _iter in 1..8 {
        let section = file_iter.next().unwrap();
        let line_reader = section.split('\n').skip(1).take_while(|x| !x.is_empty());
        let mut ranges: Vec<MapRange> = Vec::new();
        for line in line_reader {
            let mut line_numbers = line.split(' ').map(|x| x.parse::<usize>().unwrap());
            let range = MapRange {
                dest_start: line_numbers.next().unwrap(),
                source_start: line_numbers.next().unwrap(),
                length: line_numbers.next().unwrap(),
            };

            ranges.push(range);
        }

        let mut new_seeds = Vec::with_capacity(seeds.len());
        for seed in seeds {
            new_seeds.extend(seed.partition_by(&ranges))
        }
        seeds = new_seeds;
    }
    seeds.iter().fold(usize::MAX, |current_min, seed_range| {
        min(current_min, seed_range.start)
    })
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input_file};

    const P1SAMPLE01_ANSWER: usize = 35;
    const P2SAMPLE01_ANSWER: usize = 46;

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
