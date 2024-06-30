#![allow(unused)]

pub struct LatLon {
    lat: f64,
    lon: f64,
    unit: Unit,
}
impl LatLon {
    pub fn new<T: Into<f64>>(lat: T, lon: T) -> Self {
        let lat = lat.into();
        let lon = lon.into();
        let unit = Unit::Degree;
        Self { lat, lon, unit }
    }
    pub fn in_unit(self, unit: Unit) -> Self {
        self
    }
    pub fn in_tokyo(self) -> Tokyo {
        Tokyo(self)
    }
}

pub struct Tokyo(LatLon);
impl Tokyo {
    pub fn to_jgd2000(self) -> Jgd2000 {
        Jgd2000(self.0)
    }
}

pub struct Jgd2000(LatLon);
impl Jgd2000 {
    pub fn dest(self) -> (f64, f64) {
        (0., 0.)
    }
}

pub enum Unit {
    Degree,
    Second,
}

fn usage() {
    LatLon::new(1., 2.)
        .in_unit(Unit::Second)
        .in_tokyo()
        .to_jgd2000()
        .dest();
}
