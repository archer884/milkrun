use crate::param::{ParseAltitudeError, ParseRatioError};
use std::error::Error;
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

impl From<ParseFloatError> for BuildParametersError {
    fn from(e: ParseFloatError) -> Self {
        BuildParametersError::Period(e)
    }
}

impl From<ParseRatioError> for BuildParametersError {
    fn from(e: ParseRatioError) -> Self {
        BuildParametersError::Ratio(e)
    }
}

impl Display for BuildParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildParametersError::Altitude(e) => write!(f, "{}", e),
            BuildParametersError::Period(e) => write!(f, "Bad orbital period: {}", e),
            BuildParametersError::Ratio(e) => write!(f, "{}", e),
            BuildParametersError::Body(name) => write!(f, "Body not found: {}", name),
        }
    }
}

impl Error for BuildParametersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BuildParametersError::Altitude(e) => Some(e),
            BuildParametersError::Period(e) => Some(e),
            BuildParametersError::Ratio(e) => Some(e),
            _ => None,
        }
    }
}
