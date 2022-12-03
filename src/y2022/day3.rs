use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_rust::utils::ProblemError;
use itertools::Itertools;

pub fn parse_input(filename: &str) -> Result<Vec<Vec<u32>>, ProblemError> {
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);

    let bags: Vec<_> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|c| c.chars().filter_map(item_score).collect())
        .collect();

    /*.map(|line| {
        let (pocket1, pocket2) = line.split_at(line.len() / 2);
        (
            BTreeSet::from_iter(pocket1.chars()),
            BTreeSet::from_iter(pocket2.chars()),
        )
    })*/
    //.collect();

    Ok(bags)
}

fn item_score(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - 96),
        'A'..='Z' => Some(c as u32 - 38),
        _ => None,
    }
}

pub fn solve_part_1(bags: &Vec<Vec<u32>>) -> u32 {
    bags.iter()
        .filter_map(|items| {
            let (p1, p2) = items.split_at(items.len() / 2);

            let pocket1 = BTreeSet::from_iter(p1.to_owned());
            let pocket2 = BTreeSet::from_iter(p2.to_owned());
            pocket1.intersection(&pocket2).next().copied()
        })
        .sum()
}

pub fn solve_part_2(bags: &Vec<Vec<u32>>) -> u32 {
    bags.iter()
        .tuples()
        .filter_map(|(x, y, z)| {
            let bag1 = BTreeSet::from_iter(x.to_owned());
            let bag2 = BTreeSet::from_iter(y.to_owned());
            let bag3 = BTreeSet::from_iter(z.to_owned());

            bag1.intersection(&bag2)
                .copied()
                .collect::<BTreeSet<_>>()
                .intersection(&bag3)
                .next()
                .copied()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input("input/2022/day3_mini.txt").expect("");
        assert_eq!(solve_part_1(&input), 157);
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input/2022/day3_mini.txt").expect("");
        assert_eq!(solve_part_2(&input), 70);
    }
}
