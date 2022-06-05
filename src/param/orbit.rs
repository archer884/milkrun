use super::Ratio;

const EARTH_RADIUS: f64 = 6_378_137.0;

const KERBIN_RADIUS: f64 = 600_000.0;

/// Description of a celestial body
///
/// Used to create orbits using altitude figures relative to ground level, since Ap and Pe must be
/// given in absolute terms for orbital calculations--i.e., altitude must be measured from the
/// center of the earth rather than from the earth's surface.
pub struct Body {
    radius: f64,
}

impl Body {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    /// Creates an orbital description for this body using provided ALTITUDE figures
    ///
    /// ALTITUDE refers to height above mean sea level. This is useful for games like Kerbal Space
    /// Program, wherein all telemetry provided for spacecraft is with reference to the mean
    /// surface of the orbited body rather than to the body's center of mass.
    pub fn orbit(&self, ap: f64, pe: f64, period: f64) -> Orbit {
        Orbit {
            ap: ap + self.radius,
            pe: pe + self.radius,
            period,
            radius: Some(self.radius),
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Body::new(EARTH_RADIUS)
    }
}

/// Description of an orbit
#[derive(Clone, Debug)]
pub struct Orbit {
    /// Ap (apoapsis) in meters
    ap: f64,

    /// Pe (periapsis) in meters
    pe: f64,

    /// Orbital period in hours
    period: f64,

    /// Body radius
    ///
    /// This value is used only for producing final altitude values
    radius: Option<f64>,
}

impl Orbit {
    /// Creates an orbital description without reference to a specific body
    ///
    /// NOTE: Ap and Pe values provided to this function MUST BE ABSOLUTE.
    pub fn new(ap: f64, pe: f64, period: f64) -> Self {
        Self {
            ap,
            pe,
            period,
            radius: None,
        }
    }

    pub fn resonant_ap(&self, resonance: Ratio) -> f64 {
        let semimajor_axis = self.semimajor_axis();
        let relationship = semimajor_axis.powi(3) / self.period.powi(2);

        let resonant_ratio: f64 = resonance.into();
        let desired_period = self.period * resonant_ratio;
        let desired_semimajor_axis = (relationship * desired_period.powi(2)).cbrt();
        let desired_ap = self.ap - (semimajor_axis * 2.0 - desired_semimajor_axis * 2.0);

        self.radius
            .map(|radius| desired_ap - radius)
            .unwrap_or(desired_ap)
    }

    pub fn resonant_pe(&self, resonance: Ratio) -> Option<f64> {
        let semimajor_axis = self.semimajor_axis();
        let relationship = semimajor_axis.powi(3) / self.period.powi(2);

        let resonant_ratio: f64 = resonance.into();
        let desired_period = self.period * resonant_ratio;
        let desired_semimajor_axis = (relationship * desired_period.powi(2)).cbrt();

        // The math for this only works when we're making the orbital period shorter, so the
        // desired semi-major axis must be smaller than the actual semi-major axis.

        if desired_semimajor_axis > semimajor_axis {
            return None;
        }

        // An additional failure mode for this calculation is if the desired Pe is less than
        // the minimum allowable Pe, which is equal to zero in the ideal case or, in the real
        // world, the mean surface altitude of the orbited body. In actuality, this will be
        // equal to the height of the *atmosphere* of the orbited body, but we do not store that.

        let desired_pe = self.pe - (semimajor_axis * 2.0 - desired_semimajor_axis * 2.0);
        if desired_pe > self.radius.unwrap_or_default() {
            Some(desired_pe)
        } else {
            None
        }
    }

    /// Calculates the semi-major axis of the orbit
    ///
    /// The major axis of an ellipse (all orbits are technically elliptical) is its longest
    /// diameter. The semi-major axis is effectively the radius of that diameter. This value must
    /// be calculated with reference to Ap and Pe, because the length of the semi-major axis has
    /// nothing to do with the location of the center of the orbited body.
    fn semimajor_axis(&self) -> f64 {
        (self.ap + self.pe) / 2.0
    }
}
