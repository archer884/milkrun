// FIXME: Print Δv requirements, if possible?
// I believe this will require that the user provide the minor axis for calculation. With that, it
// will be possible to determine average orbital speed (by calculating the perimiter of the orbit
// itself and simply dividing perimeter in meters by the period in hours to get meters/hour). What
// we do with orbital speed, exactly, I'm not sure yet.

// What about having the calculations produce a resonant orbit rather than a resonant Ap/Pe? Then
// the Δv could be derived by comparing the two orbits, provided that the orbital description(s)
// are made to contain the necessary information. (I still think I just add the semi-minor axis.)

mod command;
mod error;
mod param;

use command::Command;

type Result<T, E = error::BuildParametersError> = std::result::Result<T, E>;

fn main() {
    let (ratio, orbit) = match Command::from_args().build() {
        Ok(params) => params,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if ratio.is_greater_than_one() {
        println!("{:.02}", orbit.resonant_ap(ratio));
    } else {
        match orbit.resonant_pe(ratio) {
            Some(pe) => println!("{pe:.02}"),
            None => eprintln!("resulting orbit is impossible (Pe too low)"),
        }
    }
}
