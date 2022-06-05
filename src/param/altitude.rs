use std::{
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

use super::ParseTwoErr;

#[derive(Debug)]
pub struct Altitude {
    pub ap: f64,
    pub pe: f64,
}

impl FromStr for Altitude {
    type Err = ParseAltitudeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const KEOSYNCHRONOUS_ALTITUDE: f64 = 2_863_330.0;

        match s.to_lowercase().as_ref() {
            "keo" | "keosynchronous" | "geosynchronous" | "geostationary" => Ok(Altitude {
                ap: KEOSYNCHRONOUS_ALTITUDE,
                pe: KEOSYNCHRONOUS_ALTITUDE,
            }),

            _ => {
                // If altitude is given as a single value...
                if let Ok(altitude) = s.parse() {
                    return Ok(Altitude {
                        ap: altitude,
                        pe: altitude,
                    });
                }

                let (ap, pe) = super::parse_two(s)?;
                Ok(Altitude { ap, pe })
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseAltitudeError(ParseTwoErr<std::num::ParseFloatError>);

impl From<ParseTwoErr<std::num::ParseFloatError>> for ParseAltitudeError {
    fn from(e: ParseTwoErr<std::num::ParseFloatError>) -> Self {
        ParseAltitudeError(e)
    }
}

impl Display for ParseAltitudeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad altitude: {}", self.0)
    }
}

impl Error for ParseAltitudeError {}
