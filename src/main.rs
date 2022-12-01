mod y2022;

use aoc_rust::utils::ProblemError;

fn main() -> Result<(), ProblemError> {
    println!("=== AOC 2022 ===");

    // ------------------------------------------------------------------------
    println!("> Day 1");
    let input = y2022::day1::parse_input("input/2022/day1.txt")?;
    println!(
        "- part 1: {}",
        y2022::day1::get_max_weight(&input).ok_or("No value in the input")?
    );
    println!(
        "- part 2: {}",
        y2022::day1::get_top_three_weights(&input).ok_or("Not enough values in the input")?
    );
    // ------------------------------------------------------------------------

    Ok(())
}
