// FIXME: Allow named altitudes like "geostationary"
// FIXME: Print delta v requirements, if possible? (may require data I'm not getting right now)
// FIXME: make this work for inverted ratios--3:2 as well as 2:3
// FIXME: abstract out the parsing code instead of just repeating it for both altitude and ratio

mod altitude;
mod command;
mod error;
mod orbit;
mod ratio;

type Result<T, E = error::BuildParametersError> = std::result::Result<T, E>;

fn main() {
    // const KEOSYNCHRONOUS_ALTITUDE: f64 = 2_863_330.0;

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

    match command::Command::from_args().build() {
        Err(e) => println!("{:?}", e),
        Ok((ratio, orbit)) => println!("{}", orbit.resonant_periapsis(ratio.resonance()).unwrap()),
    }
}
