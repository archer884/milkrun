use std::{
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

use super::ParseTwoErr;

#[derive(Clone, Copy, Debug)]
pub struct Ratio {
    pub numer: i32,
    pub denom: i32,
}

impl Ratio {
    pub fn new(numer: i32, denom: i32) -> Self {
        Self { numer, denom }
    }

    pub fn is_greater_than_one(&self) -> bool {
        self.numer > self.denom
    }
}

impl From<Ratio> for f64 {
    fn from(ratio: Ratio) -> Self {
        let numer = f64::from(ratio.numer);
        let denom = f64::from(ratio.denom);
        numer / denom
    }
}

impl FromStr for Ratio {
    type Err = ParseRatioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = super::parse_two(s)?;
        Ok(Ratio {
            numer: left,
            denom: right,
        })
    }
}

#[derive(Debug)]
pub struct ParseRatioError(ParseTwoErr<std::num::ParseIntError>);

impl From<ParseTwoErr<std::num::ParseIntError>> for ParseRatioError {
    fn from(e: ParseTwoErr<std::num::ParseIntError>) -> Self {
        ParseRatioError(e)
    }
}

impl Display for ParseRatioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad ratio: {}", self.0)
    }
}

impl Error for ParseRatioError {}
