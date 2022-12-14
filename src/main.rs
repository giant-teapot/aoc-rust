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
    println!("> Day 2");
    let input = y2022::day2::parse_input_part1("input/2022/day2.txt")?;
    println!("- part 1: {}", y2022::day2::get_total_score_part1(&input));
    let input = y2022::day2::parse_input_part2("input/2022/day2.txt")?;
    println!("- part 2: {}", y2022::day2::get_total_score_part2(&input));
    // ------------------------------------------------------------------------
    println!("> Day 3");
    let input = y2022::day3::parse_input("input/2022/day3.txt")?;
    println!("- part 1: {}", y2022::day3::solve_part_1(&input));
    println!("- part 2: {}", y2022::day3::solve_part_2(&input));
    // ------------------------------------------------------------------------
    println!("> Day 4");
    let input = y2022::day4::parse_input("input/2022/day4.txt")?;
    println!("- part 1: {}", y2022::day4::solve_part_1(&input));
    println!("- part 2: {}", y2022::day4::solve_part_2(&input));
    // ------------------------------------------------------------------------
    println!("> Day 5");
    let input = y2022::day5::parse_input("input/2022/day5.txt")?;
    println!("- part 1: {}", y2022::day5::solve_part_1(input));
    let input = y2022::day5::parse_input("input/2022/day5.txt")?;
    println!("- part 2: {}", y2022::day5::solve_part_2(input));
    // ------------------------------------------------------------------------
    println!("> Day 6");
    let input = y2022::day6::parse_input("input/2022/day6.txt")?;
    println!(
        "- part 1: {}",
        y2022::day6::solve_part_1(&input)
            .ok_or_else(|| ProblemError::CustomError("No solution found.".to_string()))?
    );
    println!(
        "- part 1: {}",
        y2022::day6::solve_part_2(&input)
            .ok_or_else(|| ProblemError::CustomError("No solution found.".to_string()))?
    );

    Ok(())
}
