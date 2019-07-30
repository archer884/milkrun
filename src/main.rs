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
    match command::Command::from_args().build() {
        Err(e) => println!("{:?}", e),
        Ok((ratio, orbit)) => println!("{}", orbit.resonant_periapsis(ratio.resonance()).unwrap()),
    }
}
