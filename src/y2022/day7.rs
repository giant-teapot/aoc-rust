use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use aoc_rust::utils::ProblemError;

use regex::Regex;

#[derive(Debug)]
enum Command {
    Cd(PathBuf),
    Ls,
}

type CommandExec = (Command, Option<Vec<String>>);

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Cd(x) => write!(f, "cd {:?}", x),
            Command::Ls => write!(f, "ls"),
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = ProblemError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with("$") {
            return Err(ProblemError::CustomError("".to_string()));
        }

        let re = Regex::new(r"^\$\s+(?P<cmd_name>\w+)(\s+(?P<args>.+))?")
            .map_err(|e| ProblemError::CustomError(format!("{:?}", e)))?;
        let caps = re
            .captures(value)
            .ok_or_else(|| ProblemError::CustomError(format!("Invalid command: '{}'", value)))?;

        match &caps["cmd_name"] {
            "cd" => {
                let path = PathBuf::from_str(&caps["args"])
                    .map_err(|e| ProblemError::CustomError(format!("{:?}", e)))?;
                Ok(Command::Cd(path))
            }
            _ => Ok(Command::Ls),
        }
    }
}

pub fn parse_input(filename: &str) -> Result<(), ProblemError> {
    let input = File::open(filename)?;
    let reader = BufReader::new(input);

    for line in reader.lines().filter_map(|x| x.ok()) {
        if let Ok(cmd) = Command::try_from(line.as_str()) {
            let _exec: CommandExec = match cmd {
                Command::Cd(x) => (Command::Cd(x), None),
                Command::Ls => {
                    // Todo
                    //peeking_take_while(|l| !l.starts_with("$"))
                    (Command::Ls, Some(vec!["".to_string()]))
                }
            };
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {}
}
