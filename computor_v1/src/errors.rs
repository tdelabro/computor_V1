use std::error;
use std::fmt;

#[derive(Debug)]
pub struct BadFormat;

impl error::Error for BadFormat {}

impl fmt::Display for BadFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Argument uncorectly formated.")
    }
}

#[derive(Debug)]
pub struct DegreeTooHigh {
	pub degree: u8,
}

impl error::Error for DegreeTooHigh {}

impl fmt::Display for DegreeTooHigh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The polynomial degree equals {}, which is strictly greater than 2. It couldn't be solved.", self.degree)
    }
}

pub enum ParsingError {
	FormatError(BadFormat),
	DegreeError(DegreeTooHigh),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParsingError::FormatError(e) => write!(f, "{}", e),
			ParsingError::DegreeError(e) => write!(f, "{}", e),
		}
    }
}

impl From<BadFormat> for ParsingError {
    fn from(error: BadFormat) -> Self {
        ParsingError::FormatError(error)
    }
}

impl From<DegreeTooHigh> for ParsingError {
    fn from(error: DegreeTooHigh) -> Self {
        ParsingError::DegreeError(error)
    }
}

#[derive(Debug)]
pub struct MissingArgument;

impl error::Error for MissingArgument {}

impl fmt::Display for MissingArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No argument have been passed.")
    }
}

#[derive(Debug)]
pub struct TooManyArguments;

impl error::Error for TooManyArguments {}

impl fmt::Display for TooManyArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Too many arguments have been passed. Only one is allowed.")
    }
}
