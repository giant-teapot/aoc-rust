use std::{
    fs::File,
    io::{BufReader, Read},
};

use aoc_rust::utils::ProblemError;
use itertools::Itertools;

pub fn parse_input(filename: &str) -> Result<String, ProblemError> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    Ok(input)
}

fn unique_window_end(buffer: &str, window_size: usize) -> Option<usize> {
    buffer
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
        .filter(|(_, xs)| xs.iter().all_unique())
        .map(|(i, _)| i)
        .next()
        .map(|i| i + window_size)
}

pub fn solve_part_1(buffer: &str) -> Option<usize> {
    unique_window_end(buffer, 4)
}

pub fn solve_part_2(buffer: &str) -> Option<usize> {
    unique_window_end(buffer, 14)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(solve_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(solve_part_1("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(solve_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(solve_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(solve_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(solve_part_2("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(solve_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(solve_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
