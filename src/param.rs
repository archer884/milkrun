mod altitude;
mod orbit;
mod ratio;

pub use altitude::{Altitude, ParseAltitudeError};
pub use orbit::Orbit;
pub use ratio::{Ratio, ParseRatioError};

use std::num::ParseFloatError;
use std::fmt::{self, Display};
use std::error::Error;

pub fn parse_two_part_float(s: &str) -> Result<(f64, f64), ParseTwoPartFloatError> {
    let mut parts = s.split(|u: char| !u.is_ascii_digit());

    let left = parts.next()
        .ok_or(ParseTwoPartFloatError::MissingSegment)?
        .parse()?;
    let right = parts.next()
        .ok_or(ParseTwoPartFloatError::MissingSegment)?
        .parse()?;

    match parts.next() {
        Some(_) => Err(ParseTwoPartFloatError::TooManyParts),
        None => Ok((left, right)),
    }
}

#[derive(Debug)]
pub enum ParseTwoPartFloatError {
    Float(ParseFloatError),
    MissingSegment,
    TooManyParts,
}

impl From<ParseFloatError> for ParseTwoPartFloatError {
    fn from(e: ParseFloatError) -> Self {
        ParseTwoPartFloatError::Float(e)
    }
}

impl Display for ParseTwoPartFloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseTwoPartFloatError::Float(e) => write!(f, "{}", e),
            ParseTwoPartFloatError::MissingSegment => f.write_str("invalid format (missing segment)"),
            ParseTwoPartFloatError::TooManyParts => f.write_str("invalid format (too many segments)"),
        }
    }
}

impl Error for ParseTwoPartFloatError {}
