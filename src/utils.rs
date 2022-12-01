use std::fmt::Display;

#[derive(Debug)]
pub enum ProblemError {
    InputFileError(std::io::Error),
    IntValueError(std::num::ParseIntError),
    CustomError(String),
}

impl Display for ProblemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for ProblemError {
    fn from(e: std::io::Error) -> Self {
        ProblemError::InputFileError(e)
    }
}

impl From<std::num::ParseIntError> for ProblemError {
    fn from(e: std::num::ParseIntError) -> Self {
        ProblemError::IntValueError(e)
    }
}

impl From<&str> for ProblemError {
    fn from(e: &str) -> Self {
        ProblemError::CustomError(e.to_string())
    }
}
