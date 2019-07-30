// FIXME: Print delta v requirements, if possible? (may require data I'm not getting right now)
// FIXME: abstract out the parsing code instead of just repeating it for both altitude and ratio

mod command;
mod error;
mod param;

type Result<T, E = error::BuildParametersError> = std::result::Result<T, E>;

fn main() {
    match command::Command::from_args().build() {
        Err(e) => println!("{:?}", e),
        Ok((ratio, orbit)) => {
            if ratio.is_greater_than_one() {
                println!("{}", orbit.resonant_apoapsis(ratio.resonance()).unwrap());
            } else {
                println!("{}", orbit.resonant_periapsis(ratio.resonance()).unwrap());
            }
        }
    }
}
