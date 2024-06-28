#![allow(unused)]

pub struct LatLon<T> {
    degrees: T,
}
impl LatLon<Degrees> {
    pub fn from_degrees(lat: f64, lon: f64) -> Self {
        let degrees = Degrees { lat, lon };
        Self { degrees }
    }
    pub fn from_secs(lat: f64, lon: f64) -> Self {
        let degrees = Degrees { lat, lon };
        Self { degrees }
    }
    pub fn from_tokyo(self) -> LatLon<Tokyo> {
        let degrees = Tokyo(self.degrees);
        LatLon { degrees }
    }
}
impl LatLon<Tokyo> {
    pub fn to_jgd2000(self) -> LatLon<Jgd2000> {
        let degrees = Jgd2000(self.degrees.0);
        LatLon { degrees }
    }
}
impl LatLon<Jgd2000> {
    pub fn to_jgd2011(self) -> LatLon<Jgd2011> {
        let degrees = Jgd2011(self.degrees.0);
        LatLon { degrees }
    }
}
impl<T: Geodetic> LatLon<T> {
    pub fn degrees(self) -> (f64, f64) {
        let degrees = self.degrees.degrees();
        (degrees.lat, degrees.lon)
    }
    pub fn to_secs(self) -> (f64, f64) {
        let degrees = self.degrees.degrees();
        (degrees.lat, degrees.lon)
    }
}

#[doc(hidden)]
pub trait Geodetic {
    fn degrees(self) -> Degrees;
}
pub struct Tokyo(Degrees);
impl Geodetic for Tokyo {
    fn degrees(self) -> Degrees {
        self.0
    }
}
pub struct Jgd2000(Degrees);
impl Geodetic for Jgd2000 {
    fn degrees(self) -> Degrees {
        self.0
    }
}
pub struct Jgd2011(Degrees);
impl Geodetic for Jgd2011 {
    fn degrees(self) -> Degrees {
        self.0
    }
}

pub struct Degrees {
    lat: f64,
    lon: f64,
}

mod tests {
    use super::LatLon;

    fn usage() {
        let (lat, lon) = LatLon::from_secs(0., 1.)
            .from_tokyo()
            .to_jgd2000()
            .to_jgd2011()
            .to_secs();
    }
}
