/// An orbit described in terms of its apoapsis, periapsis, and period.
///
/// Ap and Pe are expressed in meters, with period being expressed in hours. To discover the
/// semimajor axis of an orbit, it is necessary to have additionally the radius of the orbited
/// body, because Kerbal Space Program calculates Ap and Pe as altitude above the mean "sea level"
/// of the orbited body.
#[derive(Debug)]
pub struct Orbit {
    ap: f64,
    pe: f64,
    period: f64,
    body_radius: f64,
}

impl Orbit {
    pub fn new(ap: f64, pe: f64, period: f64, body_radius: f64) -> Self {
        Self {
            ap,
            pe,
            period,
            body_radius,
        }
    }

    /// Calculates the necessary periapsis to achieve the desired orbital resonance.
    ///
    /// Result is optional because it is possible that the necessary reduction in length of the
    /// semimajor axis could be greater than the current periapsis.
    pub fn resonant_periapsis(&self, resonance: f64) -> Option<f64> {
        let semimajor_axis = self.semimajor_axis();
        let relationship = semimajor_axis.powi(3) / self.period.powi(2);

        let desired_period = self.period * resonance;
        let desired_semimajor_axis = (relationship * desired_period.powi(2)).cbrt();

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

    // I have no idea if any of this math will work out or not, but the intention here is to
    // employ this method in the event our desired resonance is > 1.
    pub fn resonant_apoapsis(&self, resonance: f64) -> Option<f64> {
        let semimajor_axis = self.semimajor_axis();
        let relationship = semimajor_axis.powi(3) / self.period.powi(2);

        let desired_period = self.period * resonance;
        let desired_semimajor_axis = (relationship * desired_period.powf(2.0)).cbrt();

        let desired_ap = self.ap - (semimajor_axis * 2.0 - desired_semimajor_axis * 2.0);
        Some(desired_ap)
    }

    fn semimajor_axis(&self) -> f64 {
        (self.ap + self.pe) / 2.0 + self.body_radius
    }
}
