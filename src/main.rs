// FIXME: Print delta v requirements, if possible? (may require data I'm not getting right now)

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
