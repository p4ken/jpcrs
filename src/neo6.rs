#![allow(unused)]

pub struct Tokyo;
impl Tokyo {
    pub fn to_jgd2000(self) -> Jgd2000 {
        Jgd2000
    }
}
impl Degrees for Tokyo {
    fn with_degrees(degrees: impl Into<LatLon>) -> Self {
        Self
    }
    fn degrees(&self) -> LatLon {
        LatLon(0., 0.)
    }
}

pub struct Jgd2000;
impl Jgd2000 {
    pub fn to_jgd2011(self) -> Jgd2011 {
        Jgd2011
    }
}
pub struct Jgd2011;
impl Degrees for Jgd2011 {
    fn with_degrees(degrees: impl Into<LatLon>) -> Self {
        Self
    }
    fn degrees(&self) -> LatLon {
        LatLon(0., 0.)
    }
}

// TODO sealed
pub trait Degrees: Sized {
    // TODO: Into要らないかも
    fn with_degrees(degrees: impl Into<LatLon>) -> Self;
    fn with_secs(secs: LatLon) -> Self {
        Self::with_degrees(secs)
    }
    fn degrees(&self) -> LatLon;
    fn secs(&self) -> LatLon {
        self.degrees()
    }
}
pub struct LatLon<T = f64>(T, T);
impl LatLon {
    pub fn from_rev(xy: impl Into<[f64; 2]>) -> Self {
        let [x, y] = xy.into();
        Self(y, x)
    }
    pub fn into_rev<T: From<[f64; 2]>>(&self) -> T {
        [self.lon(), self.lat()].into()
    }
    pub fn lat(&self) -> f64 {
        self.0
    }
    pub fn lon(&self) -> f64 {
        self.1
    }
}
// impl<T: Into<f64>> From<[T; 2]> for LatLon {
//     fn from([lat, lon]: [T; 2]) -> Self {
//         Self(lat.into(), lon.into())
//     }
// }
// impl From<LatLon> for [f64; 2] {
//     fn from(value: LatLon) -> Self {
//         [value.lat(), value.lon()]
//     }
// }

#[cfg(test)]
pub fn usage() {
    use geo::Point;

    //　やはりベストか
    let LatLon(lat, lon) = Tokyo::with_degrees(LatLon(0., 0.))
        .to_jgd2000()
        .to_jgd2011()
        .degrees();

    let LatLon(lat, lon) = Tokyo::with_secs(LatLon(0., 0.))
        .to_jgd2000()
        .to_jgd2011()
        .secs();

    let mut p = Point::new(1., 2.);
    p = Tokyo::with_degrees(LatLon::from_rev(p))
        .to_jgd2000()
        .to_jgd2011()
        .degrees()
        .into_rev();

    let mut p = Point::new(1., 2.);
    let LatLon(y, x) = Tokyo::with_degrees(LatLon(p.y(), p.x()))
        .to_jgd2000()
        .to_jgd2011()
        .degrees();
    p = Point::new(x, y);
}
