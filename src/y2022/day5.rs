use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Read},
};

use aoc_rust::utils::ProblemError;
use itertools::Itertools;

type Instruction = (usize, usize, usize);

fn read_stacks_slice(input_line: &str) -> Vec<Option<char>> {
    input_line
        .chars()
        .chunks(4)
        .into_iter()
        .map(|x| {
            x.collect::<String>()
                .trim_matches(|c: char| !c.is_ascii_alphabetic())
                .chars()
                .next()
        })
        .collect()
}

pub fn parse_input(
    filename: &str,
) -> Result<(Vec<VecDeque<char>>, Vec<Instruction>), ProblemError> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let stack_slices = reader
        .by_ref()
        .lines()
        .filter_map(|line| line.ok())
        .map_while(|line| match line.trim() {
            "" => None,
            _ => Some(read_stacks_slice(&line)),
        })
        .collect::<Vec<_>>();

    let mut stacks: Vec<VecDeque<char>> =
        vec![VecDeque::new(); stack_slices.get(0).map_or(0, |s| s.len())];
    for slice in &stack_slices {
        for (i, crate_label) in slice.iter().enumerate() {
            if let Some(label) = crate_label {
                stacks[i].push_front(*label);
            }
        }
    }

    let instructions = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let mut splits = line.split_whitespace();
            Some((
                splits.nth(1).and_then(|x| x.parse::<usize>().ok())?,
                splits.nth(1).and_then(|x| x.parse::<usize>().ok())?,
                splits.nth(1).and_then(|x| x.parse::<usize>().ok())?,
            ))
        })
        .collect::<Vec<_>>();

    Ok((stacks, instructions))
}

pub fn solve_part_1(input: (Vec<VecDeque<char>>, Vec<Instruction>)) -> String {
    let (mut stacks, instructions) = input;

    for (count, from, to) in instructions {
        for _ in 0..count {
            let value = stacks.get_mut(from - 1).and_then(|s| s.pop_back()).unwrap();
            if let Some(s) = stacks.get_mut(to - 1) {
                s.push_back(value)
            }
        }
    }

    stacks.iter().filter_map(|s| s.back()).collect::<String>()
}

pub fn solve_part_2(input: (Vec<VecDeque<char>>, Vec<Instruction>)) -> String {
    let (mut stacks, instructions) = input;

    for (count, from, to) in instructions {
        let stack_from = stacks.get_mut(from - 1).unwrap();
        let mut transit_stack = VecDeque::new();
        for _ in 0..count {
            transit_stack.push_back(stack_from.pop_back());
        }

        let stack_to = stacks.get_mut(to - 1).unwrap();
        while let Some(x) = transit_stack.pop_back().flatten() {
            stack_to.push_back(x);
        }

        // println!("------------------------------------");
        // println!("\t{} -> {}", from - 1, to - 1);
        // for (i, s) in stacks.iter().enumerate() {
        //     println!("[{}]: {:?}", i, s);
        // }
    }

    stacks.iter().filter_map(|s| s.back()).collect::<String>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input("input/2022/day5_mini.txt").expect("");
        assert_eq!(solve_part_1(input), "CMZ");
    }

    #[test]
    fn test_part_2() {
        let input = parse_input("input/2022/day5_mini.txt").expect("");
        assert_eq!(solve_part_2(input), "MCD");
    }
}
