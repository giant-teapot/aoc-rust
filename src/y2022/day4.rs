use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_rust::utils::ProblemError;

type Section = (i32, i32);

pub fn parse_input(filename: &str) -> Result<Vec<(Section, Section)>, ProblemError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let section_pairs = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let numbers = line
                .splitn(4, |c| c == '-' || c == ',')
                .filter_map(|number| number.parse::<i32>().ok())
                .collect::<Vec<_>>();

            match numbers.len() {
                4 => Some(((numbers[0], numbers[1]), (numbers[2], numbers[3]))),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    Ok(section_pairs)
}

fn contained(a: &Section, b: &Section) -> bool {
    let (x1, y1) = a;
    let (x2, y2) = b;

    (x1 <= x2 && y2 <= y1) || (x2 <= x1 && y1 <= y2)
}

pub fn solve_part_1(section_pairs: &[(Section, Section)]) -> usize {
    section_pairs
        .iter()
        .filter(|(a, b)| contained(a, b))
        .count()
}

fn overlaps(a: &Section, b: &Section) -> bool {
    let (x1, y1) = a;
    let (x2, y2) = b;

    y1 >= x2 && x1 <= y2
}

pub fn solve_part_2(section_pairs: &[(Section, Section)]) -> usize {
    section_pairs.iter().filter(|(a, b)| overlaps(a, b)).count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input("input/2022/day4_mini.txt").expect("");
        assert_eq!(solve_part_1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input/2022/day4_mini.txt").expect("");
        assert_eq!(solve_part_2(&input), 4);
    }
}
