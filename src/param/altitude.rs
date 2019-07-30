use crate::error::ParseAltitudeError;
use std::str::FromStr;

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
            "keosynchronous" | "geosynchronous" | "geostationary" => Ok(Altitude {
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

                let mut parts = s.trim().split('x');

                // FIXME: this ignores additional altitude segments.
                Ok(Altitude {
                    ap: parts
                        .next()
                        .ok_or(ParseAltitudeError::MissingSegment)?
                        .parse()?,
                    pe: parts
                        .next()
                        .ok_or(ParseAltitudeError::MissingSegment)?
                        .parse()?,
                })
            }
        }
    }
}
