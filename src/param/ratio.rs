use crate::error::ParseRatioError;
use std::str::FromStr;

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
        let mut parts = s.trim().split(':');

        // FIXME: this ignores additional ratio segments.
        Ok(Ratio {
            left: parts
                .next()
                .ok_or(ParseRatioError::MissingSegment)?
                .parse()?,
            right: parts
                .next()
                .ok_or(ParseRatioError::MissingSegment)?
                .parse()?,
        })
    }
}
