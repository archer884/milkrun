const KERBIN_RADIUS: f64 = 600_000.0; // meters

/// An orbit described in terms of its apoapsis, periapsis, and period.
///
/// Ap and Pe are expressed in meters, with period being expressed in hours. To discover the
/// semimajor axis of an orbit, it is necessary to have additionally the radius of the orbited
/// body, because Kerbal Space Program calculates Ap and Pe as altitude above the mean "sea level"
/// of the orbited body.
struct Orbit {
    ap: f64,
    pe: f64,
    period: f64, // Is this field important?
    body_radius: f64,
}

impl Orbit {
    fn new(ap: f64, pe: f64, period: f64) -> Self {
        Self {
            ap,
            pe,
            period,
            body_radius: KERBIN_RADIUS,
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

        let desired_pe = self.pe - (semimajor_axis - desired_semimajor_axis);
        if desired_pe > 0.0 {
            Some(desired_pe)
        } else {
            None
        }
    }
}

fn main() {
    const KEOSYNCHRONOUS_ALTITUDE: f64 = 2_863_330.0;

    let orbit = Orbit::new(
        KEOSYNCHRONOUS_ALTITUDE,    // Altitude in meters
        KEOSYNCHRONOUS_ALTITUDE,    // Altitude in meters
        6.0,                        // Time in hours
    );

    let resonant_periapsis = orbit.resonant_periapsis(1.0 / 3.0).expect("Fuck!");
    let four_hour_periapsis = orbit.resonant_periapsis(2.0 / 3.0).expect("Shit!");
    
    println!("{:02} meters", resonant_periapsis);
    println!("{:02} meters", four_hour_periapsis);
}
