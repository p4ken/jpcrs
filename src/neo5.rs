#![allow(unused)]

pub struct Tokyo;
impl Tokyo {
    pub fn new(lat_lon: LatLon) -> Self {
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
impl Jgd2011 {
    fn lat_lon(&self) -> LatLon {
        LatLon(0., 0.)
    }
}
// impl From<Jgd2011> for (f64, f64) {
//     fn from(crs: Jgd2011) -> Self {
//         crs.degrees()
//     }
// }
// impl AsRef<LatLon> for Jgd2011 {
//     fn as_ref(&self) -> &LatLon {
//         &LatLon(0., 0.)
//     }
// }
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
pub struct LatLon(f64, f64);
// impl Degrees for LatLon {
//     fn with_degrees(lat: f64, lon: f64) -> Self {
//         todo!()
//     }
//     fn degrees(self) -> (f64, f64) {
//         todo!()
//     }
// }

#[cfg(test)]
pub fn usage() {
    use geo::Point;

    // let (lat, lon) = Tokyo::new(LatLon(1., 2.))
    //     .to_jgd2000()
    //     .to_jgd2011()
    //     .lat_lon()
    //     .secs();
    let (lat, lon) = Tokyo::with_degrees(1., 2.)
        .to_jgd2000()
        .to_jgd2011()
        .degrees();

    let (lat, lon) = Tokyo::with_secs(1., 2.).to_jgd2000().to_jgd2011().secs();

    let mut p = Point::new(1., 2.);
    let (lat, lon) = Tokyo::with_degrees(p.y(), p.x())
        .to_jgd2000()
        .to_jgd2011()
        .degrees();
    p = Point::new(lon, lat);
}
