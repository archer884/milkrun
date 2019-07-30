use crate::{
    error::BuildParametersError,
    param::{Altitude, Orbit, Ratio},
};
use structopt::StructOpt;

const KERBIN_RADIUS: f64 = 600_000.0; // meters

#[derive(Debug, StructOpt)]
pub struct Command {
    altitude: Altitude,
    period: f64,
    ratio: Ratio,
    body: Option<String>,
}

impl Command {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }

    pub fn build(&self) -> crate::Result<(Ratio, Orbit)> {
        let Altitude { ap, pe } = self.altitude;
        let radius = match &self.body {
            None => KERBIN_RADIUS,
            Some(body) => match body.parse() {
                Ok(radius) => radius,
                Err(_) => get_body_radius(&body)?,
            },
        };

        Ok((self.ratio, Orbit::new(ap, pe, self.period, radius)))
    }
}

fn get_body_radius(body: &str) -> crate::Result<f64> {
    match body.to_lowercase().as_ref() {
        "kerbin" => Ok(KERBIN_RADIUS),

        _ => Err(BuildParametersError::Body(body.into())),
    }
}
