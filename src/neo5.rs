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
impl From<LatLon> for Tokyo {
    fn from(_: LatLon) -> Self {
        Self
    }
}
impl From<Tokyo> for LatLon {
    fn from(_: Tokyo) -> Self {
        Self
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
impl From<LatLon> for Jgd2011 {
    fn from(_: LatLon) -> Self {
        Self
    }
}
impl From<Jgd2011> for LatLon {
    fn from(_: Jgd2011) -> Self {
        Self
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
impl<T: From<LatLon> + Into<LatLon>> Degrees for T {
    fn with_degrees(lat: f64, lon: f64) -> Self {
        LatLon.into()
    }
    fn degrees(self) -> (f64, f64) {
        (0., 0.)
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
