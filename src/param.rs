use std::{
    error::Error,
    fmt::{self, Debug, Display},
    str::FromStr,
};

mod altitude;
mod orbit;
mod ratio;

pub use altitude::{Altitude, ParseAltitudeError};
pub use orbit::{Body, Orbit};
pub use ratio::{ParseRatioError, Ratio};

pub fn parse_two<T: FromStr>(s: &str) -> Result<(T, T), ParseTwoErr<T::Err>> {
    let mut parts = s.split(|u: char| !u.is_ascii_digit());

    let left = parts.next().ok_or(ParseTwoErr::MissingSegment)?.parse()?;
    let right = parts.next().ok_or(ParseTwoErr::MissingSegment)?.parse()?;

    match parts.next() {
        Some(_) => Err(ParseTwoErr::TooManyParts),
        None => Ok((left, right)),
    }
}

#[derive(Debug)]
pub enum ParseTwoErr<E> {
    Parse(E),
    MissingSegment,
    TooManyParts,
}

impl<E> From<E> for ParseTwoErr<E> {
    fn from(e: E) -> Self {
        ParseTwoErr::Parse(e)
    }
}

impl<E: Display> Display for ParseTwoErr<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseTwoErr::Parse(e) => write!(f, "{}", e),
            ParseTwoErr::MissingSegment => f.write_str("invalid format (missing segment)"),
            ParseTwoErr::TooManyParts => f.write_str("invalid format (too many segments)"),
        }
    }
}

impl<E: Debug + Display> Error for ParseTwoErr<E> {}
