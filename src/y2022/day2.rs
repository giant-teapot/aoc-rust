use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_rust::utils::ProblemError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Cissors,
}

impl TryFrom<&str> for Move {
    type Error = ProblemError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Cissors),
            x => Err(ProblemError::CustomError(format!(
                "{} is not a valid move.",
                x
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MatchOutcome {
    Loss,
    Draw,
    Win,
}

impl TryFrom<&str> for MatchOutcome {
    type Error = ProblemError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(MatchOutcome::Loss),
            "Y" => Ok(MatchOutcome::Draw),
            "Z" => Ok(MatchOutcome::Win),
            x => Err(ProblemError::CustomError(format!(
                "{} is not a valid match outcome.",
                x
            ))),
        }
    }
}

impl Move {
    fn against(&self, other: &Move) -> MatchOutcome {
        match (self, other) {
            (Move::Rock, Move::Paper) => MatchOutcome::Loss,
            (Move::Rock, Move::Cissors) => MatchOutcome::Win,
            (Move::Paper, Move::Rock) => MatchOutcome::Win,
            (Move::Paper, Move::Cissors) => MatchOutcome::Loss,
            (Move::Cissors, Move::Rock) => MatchOutcome::Loss,
            (Move::Cissors, Move::Paper) => MatchOutcome::Win,
            (a, b) => {
                if *a == *b {
                    MatchOutcome::Draw
                } else {
                    unreachable!()
                }
            }
        }
    }
}

pub fn parse_input_part1(filename: &str) -> Result<Vec<(Move, Move)>, ProblemError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let moves = reader
        .lines()
        .filter_map(|line| line.ok())
        .flat_map(|str| {
            str.split_ascii_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .flat_map(|str| Move::try_from(str.as_str()))
        .collect::<Vec<_>>();

    let pairwise_moves = moves
        .chunks(2)
        .into_iter()
        .map(|pair| (pair[0], pair[1]))
        .collect::<Vec<_>>();

    Ok(pairwise_moves)
}

fn get_match_score(outcome: &MatchOutcome) -> u32 {
    match outcome {
        MatchOutcome::Loss => 0,
        MatchOutcome::Draw => 3,
        MatchOutcome::Win => 6,
    }
}

fn get_move_score(player_move: &Move) -> u32 {
    match player_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Cissors => 3,
    }
}

pub fn get_total_score_part1(games: &[(Move, Move)]) -> u32 {
    games
        .iter()
        .map(|(adversary_move, player_move)| {
            let outcome = player_move.against(adversary_move);
            get_move_score(player_move) + get_match_score(&outcome)
        })
        .sum()
}

pub fn parse_input_part2(filename: &str) -> Result<Vec<(Move, MatchOutcome)>, ProblemError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let moves = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|str| {
            let tokens = str.split_ascii_whitespace().collect::<Vec<_>>();
            if tokens.len() != 2 {
                return None;
            }
            let adversary = Move::try_from(tokens[0]).ok()?;
            let outcome = MatchOutcome::try_from(tokens[1]).ok()?;
            Some((adversary, outcome))
        })
        .collect::<Vec<_>>();

    Ok(moves)
}

fn required_move_for_outcome(adversary: &Move, target_outcome: &MatchOutcome) -> Move {
    match (adversary, target_outcome) {
        (Move::Rock, MatchOutcome::Loss) => Move::Cissors,
        (Move::Rock, MatchOutcome::Win) => Move::Paper,

        (Move::Paper, MatchOutcome::Loss) => Move::Rock,
        (Move::Paper, MatchOutcome::Win) => Move::Cissors,

        (Move::Cissors, MatchOutcome::Loss) => Move::Paper,
        (Move::Cissors, MatchOutcome::Win) => Move::Rock,

        (x, MatchOutcome::Draw) => *x,
    }
}

pub fn get_total_score_part2(games: &[(Move, MatchOutcome)]) -> u32 {
    games
        .iter()
        .map(|(adversary_move, outcome)| {
            let player_move = required_move_for_outcome(adversary_move, outcome);
            get_match_score(outcome) + get_move_score(&player_move)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_calculation() {
        let input = parse_input_part1("input/2022/day2_mini.txt").expect("");
        assert_eq!(get_total_score_part1(&input), 15);
    }

    #[test]
    fn test_score_part2() {
        let input = parse_input_part2("input/2022/day2_mini.txt").expect("");
        assert_eq!(get_total_score_part2(&input), 12);
    }
}
