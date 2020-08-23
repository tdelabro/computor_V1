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
        write!(
            f,
            "Too many arguments have been passed. Only one is allowed."
        )
    }
}
