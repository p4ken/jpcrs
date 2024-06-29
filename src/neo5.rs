#![allow(unused)]

pub struct Tokyo;
impl Tokyo {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self
    }
    pub fn to_jgd2000(self) -> Jgd2000 {
        Jgd2000
    }
}
impl Degrees for Tokyo {
    fn with_degrees(lat: f64, lon: f64) -> Self {
        Self
    }
    fn degrees(self) -> (f64, f64) {
        (0., 0.)
    }
}

pub struct Jgd2000;
impl Jgd2000 {
    pub fn to_jgd2011(self) -> Jgd2011 {
        Jgd2011
    }
}
pub struct Jgd2011;
impl From<Jgd2011> for (f64, f64) {
    fn from(crs: Jgd2011) -> Self {
        crs.degrees()
    }
}
impl Degrees for Jgd2011 {
    fn with_degrees(lat: f64, lon: f64) -> Self {
        Self
    }
    fn degrees(self) -> (f64, f64) {
        (0., 0.)
    }
}

// TODO sealed
pub trait Degrees: Sized {
    fn with_degrees(lat: f64, lon: f64) -> Self;
    fn with_secs(lat: f64, lon: f64) -> Self {
        Self::with_degrees(lat, lon)
    }
    fn degrees(self) -> (f64, f64);
    fn secs(self) -> (f64, f64) {
        self.degrees()
    }
}
pub struct LatLon;

pub fn usage() {
    let (lat, lon) = Tokyo::new(1., 2.).to_jgd2000().to_jgd2011().into();
    let (lat, lon) = Tokyo::with_degrees(1., 2.)
        .to_jgd2000()
        .to_jgd2011()
        .degrees();
    let (lat, lon) = Tokyo::with_secs(1., 2.).to_jgd2000().to_jgd2011().secs();
}
