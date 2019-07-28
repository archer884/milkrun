use std::error;
use std::fmt::{self, Display};
use std::num::ParseFloatError;

#[derive(Debug)]
pub enum BuildParametersError {
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
pub enum ParseAltitudeError {
    MissingSegment,
    Float(ParseFloatError),
}

impl From<ParseFloatError> for ParseAltitudeError {
    fn from(e: ParseFloatError) -> Self {
        ParseAltitudeError::Float(e)
    }
}

impl Display for ParseAltitudeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bad altitude param")
    }
}

impl error::Error for ParseAltitudeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParseAltitudeError::Float(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ParseRatioError {
    MissingSegment,
    Float(ParseFloatError),
}

impl From<ParseFloatError> for ParseRatioError {
    fn from(e: ParseFloatError) -> Self {
        ParseRatioError::Float(e)
    }
}

impl Display for ParseRatioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bad ratio param")
    }
}

impl error::Error for ParseRatioError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParseRatioError::Float(e) => Some(e),
            _ => None,
        }
    }
}
