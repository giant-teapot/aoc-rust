use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

pub fn parse_input(filename: &str) -> Vec<u32> {
    let input_file = File::open(filename).expect("");
    let reader = BufReader::new(input_file);
    let mut weight_per_elf = vec![0];

    for line in reader.lines().into_iter() {
        match line {
            Ok(line) => {
                if line.chars().all(|c| c.is_ascii_whitespace()) {
                    weight_per_elf.push(0);
                } else {
                    let item_weight = u32::from_str_radix(&line, 10).expect("");
                    let current_weight = weight_per_elf.last_mut().expect("");
                    *current_weight = current_weight.add(item_weight);
                }
            }
            Err(_) => continue,
        }
    }
    weight_per_elf
}

pub fn get_max_weight(weight_per_elf: &Vec<u32>) -> Option<u32> {
    weight_per_elf.iter().max().copied()
}

pub fn get_top_three_weights(weight_per_elf: &Vec<u32>) -> u32 {
    let mut max_heap = BinaryHeap::new();
    weight_per_elf.iter().for_each(|&w| max_heap.push(w));

    let mut sum = 0;
    for _ in 0..3 {
        sum = sum + max_heap.pop().unwrap_or_default();
    }
    sum
    // With experimental:
    // max_heap.into_iter_sorted().take(3).sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let mini_input = parse_input("input/2022/day1_mini.txt");
        assert_eq!(get_max_weight(&mini_input), Some(24000));
    }

    #[test]
    fn test_part2() {
        let mini_input = parse_input("input/2022/day1_mini.txt");
        assert_eq!(get_top_three_weights(&mini_input), 45000);
    }
}
