// FIXME: Allow named altitudes like "geostationary"
// FIXME: Print delta v requirements, if possible?

#[macro_use]
extern crate clap;

use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Debug)]
struct Command {
    altitude: String,
    period: String,
    ratio: String,
    body: Option<String>,
}

impl Command {
    fn from_args() -> Self {
        let matches = clap_app!(milkrun =>
            (@arg altitude: -a --altitude +required +takes_value)
            (@arg period: -p --period +required +takes_value)
            (@arg ratio: -r --ratio +required +takes_value)
            (@arg body: -b --body +takes_value)
        ).get_matches();

        Command {
            altitude: matches.value_of("altitude").unwrap().to_owned(),
            period: matches.value_of("period").unwrap().to_owned(),
            ratio: matches.value_of("ratio").unwrap().to_owned(),
            body: matches.value_of("body").map(|s| s.to_owned()),
        }
    }

    fn build(&self) -> Result<(Ratio, Orbit), BuildParametersError> {
        let Altitude { ap, pe } = self.altitude.parse()?;
        let period = self.period.parse()?;
        let radius = match &self.body {
            None => KERBIN_RADIUS,
            Some(body) => match body.parse() {
                Ok(radius) => radius,
                Err(_) => get_body_radius(&body)?,
            }
        };

        Ok((
            self.ratio.parse()?,
            Orbit::new(ap, pe, period, radius),
        ))
    }
}

fn get_body_radius(body: &str) -> Result<f64, BuildParametersError> {
    match body.to_lowercase().as_ref() {
        "kerbin" => Ok(KERBIN_RADIUS),

        _ => Err(BuildParametersError::Body(body.into()))
    }
}

#[derive(Debug)]
enum BuildParametersError {
    Altitude(ParseAltitudeError),
    Period(ParseFloatError),
    Ratio(ParseRatioError),
    Body(String),
}

impl From<ParseAltitudeError> for BuildParametersError {
    fn from(e: ParseAltitudeError) -> Self {
        BuildParametersError::Altitude(e)
    }
}

impl From<ParseRatioError> for BuildParametersError {
    fn from(e: ParseRatioError) -> Self {
        BuildParametersError::Ratio(e)
    }
}

impl From<ParseFloatError> for BuildParametersError {
    fn from(e: ParseFloatError) -> Self {
        BuildParametersError::Period(e)
    }
}

#[derive(Debug)]
struct Ratio {
    left: f64,
    right: f64,
}

impl Ratio {
    fn resonance(&self) -> f64 {
        self.left / self.right
    }
}

#[derive(Debug)]
enum ParseRatioError {
    MissingSegment,
    Float(ParseFloatError),
}

impl From<ParseFloatError> for ParseRatioError {
    fn from(e: ParseFloatError) -> Self {
        ParseRatioError::Float(e)
    }
}

impl FromStr for Ratio {
    type Err = ParseRatioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(':');

        // FIXME: this ignores additional ratio segments.
        Ok(Ratio {
            left: parts.next().ok_or(ParseRatioError::MissingSegment)?.parse()?,
            right: parts.next().ok_or(ParseRatioError::MissingSegment)?.parse()?,
        })
    }
}

#[derive(Debug)]
struct Altitude {
    ap: f64,
    pe: f64,
}

impl FromStr for Altitude {
    type Err = ParseAltitudeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If altitude is given as a single value...
        if let Ok(altitude) = s.parse() {
            return Ok(Altitude {
                ap: altitude,
                pe: altitude,
            });
        }

        let mut parts = s.trim().split('x');

        // FIXME: this ignores additional ratio segments.
        Ok(Altitude {
            ap: parts.next().ok_or(ParseAltitudeError::MissingSegment)?.parse()?,
            pe: parts.next().ok_or(ParseAltitudeError::MissingSegment)?.parse()?,
        })
    }
}

#[derive(Debug)]
enum ParseAltitudeError {
    MissingSegment,
    Float(ParseFloatError)
}

impl From<ParseFloatError> for ParseAltitudeError {
    fn from(e: ParseFloatError) -> Self {
        ParseAltitudeError::Float(e)
    }
}

const KERBIN_RADIUS: f64 = 600_000.0; // meters

/// An orbit described in terms of its apoapsis, periapsis, and period.
///
/// Ap and Pe are expressed in meters, with period being expressed in hours. To discover the
/// semimajor axis of an orbit, it is necessary to have additionally the radius of the orbited
/// body, because Kerbal Space Program calculates Ap and Pe as altitude above the mean "sea level"
/// of the orbited body.
#[derive(Debug)]
struct Orbit {
    ap: f64,
    pe: f64,
    period: f64,
    body_radius: f64,
}

impl Orbit {
    fn new(ap: f64, pe: f64, period: f64, body_radius: f64) -> Self {
        Self {
            ap,
            pe,
            period,
            body_radius,
        }
    }

    fn semimajor_axis(&self) -> f64 {
        (self.ap + self.pe) / 2.0 + self.body_radius
    }

    /// Calculates the necessary periapsis to achieve the desired orbital resonance.
    ///
    /// Result is optional because it is possible that the necessary reduction in length of the
    /// semimajor axis could be greater than the current periapsis. Note also that
    fn resonant_periapsis(&self, resonance: f64) -> Option<f64> {
        let semimajor_axis = self.semimajor_axis();
        let relationship = semimajor_axis.powf(3.0) / self.period.powf(2.0);

        let desired_period = self.period * resonance;
        let desired_semimajor_axis = (relationship * desired_period.powf(2.0)).cbrt();

        // This math only works if we're making the orbital period shorter.
        if desired_semimajor_axis > semimajor_axis {
            return None;
        }

        let desired_pe = self.pe - (semimajor_axis * 2.0 - desired_semimajor_axis * 2.0);
        if desired_pe > 0.0 {
            Some(desired_pe)
        } else {
            None
        }
    }
}

fn main() {
    const KEOSYNCHRONOUS_ALTITUDE: f64 = 2_863_330.0;

    // let orbit = Orbit::new(
    //     KEOSYNCHRONOUS_ALTITUDE,    // Altitude in meters
    //     KEOSYNCHRONOUS_ALTITUDE,    // Altitude in meters
    //     6.0,                        // Time in hours
    //     KERBIN_RADIUS,
    // );

    // match orbit.resonant_periapsis(2.0 / 3.0) {
    //     None => println!("Impossible"),
    //     Some(pe) => println!("{:0.2}", pe),
    // }

    // This program works well and all, but I hate the command line setup I've got. Writing
    // all the -a -p -b shit is a little bit annoying.

    match Command::from_args().build() {
        Err(e) => println!("{:?}", e),
        Ok((ratio, orbit)) => println!("{}", orbit.resonant_periapsis(ratio.resonance()).unwrap()),
    }
}
