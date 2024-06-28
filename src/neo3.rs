#![allow(unused)]

pub struct Transform<T> {
    degrees: T,
}
impl Transform<LatLon> {
    pub fn from_degrees(lat: f64, lon: f64) -> Self {
        let degrees = LatLon { lat, lon };
        Self { degrees }
    }
    pub fn with_secs(lat: f64, lon: f64) -> Self {
        let degrees = LatLon { lat, lon };
        Self { degrees }
    }
    pub fn from_tokyo(self) -> Transform<Tokyo> {
        let degrees = Tokyo(self.degrees);
        Transform { degrees }
    }
}
impl Transform<Tokyo> {
    pub fn to_jgd2000(self) -> Transform<Jgd2000> {
        let degrees = Jgd2000(self.degrees.0);
        Transform { degrees }
    }
}
impl Transform<Jgd2000> {
    pub fn to_jgd2011(self) -> Transform<Jgd2011> {
        let degrees = Jgd2011(self.degrees.0);
        Transform { degrees }
    }
}
impl<T: Geodetic> Transform<T> {
    pub fn degrees(self) -> (f64, f64) {
        let degrees = self.degrees.degrees();
        (degrees.lat, degrees.lon)
    }
    pub fn secs(self) -> (f64, f64) {
        let degrees = self.degrees.degrees();
        (degrees.lat, degrees.lon)
    }
}

#[doc(hidden)]
pub trait Geodetic {
    fn degrees(self) -> LatLon;
}
pub struct Tokyo(LatLon);
impl Geodetic for Tokyo {
    fn degrees(self) -> LatLon {
        self.0
    }
}
pub struct Jgd2000(LatLon);
impl Geodetic for Jgd2000 {
    fn degrees(self) -> LatLon {
        self.0
    }
}
pub struct Jgd2011(LatLon);
impl Geodetic for Jgd2011 {
    fn degrees(self) -> LatLon {
        self.0
    }
}

pub struct LatLon {
    lat: f64,
    lon: f64,
}

mod tests {
    use super::Transform;

    fn usage() {
        let (lat, lon) = Transform::with_secs(0., 1.)
            .from_tokyo()
            .to_jgd2000()
            .to_jgd2011()
            .degrees();
    }
}
