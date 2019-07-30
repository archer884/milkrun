use super::ParseTwoPartFloatError;
use std::str::FromStr;
use std::fmt::{self, Display};
use std::error::Error;

#[derive(Copy, Clone, Debug)]
pub struct Ratio {
    left: f64,
    right: f64,
}

impl Ratio {
    pub fn is_greater_than_one(&self) -> bool {
        self.left > self.right
    }

    pub fn resonance(&self) -> f64 {
        self.left / self.right
    }
}

impl FromStr for Ratio {
    type Err = ParseRatioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = super::parse_two_part_float(s)?;
        Ok(Ratio { left, right })
    }
}

#[derive(Debug)]
pub struct ParseRatioError(ParseTwoPartFloatError);

impl From<ParseTwoPartFloatError> for ParseRatioError {
    fn from(e: ParseTwoPartFloatError) -> Self {
        ParseRatioError(e)
    }
}

impl Display for ParseRatioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad ratio: {}", self.0)
    }
}

impl Error for ParseRatioError {}
