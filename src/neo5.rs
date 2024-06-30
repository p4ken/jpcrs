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
    fn degrees<T: From<LatLon>>(self) -> T {
        LatLon(0., 0.).into()
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
    fn degrees<T: From<LatLon>>(self) -> T {
        LatLon(0., 0.).into()
    }
}

// TODO sealed
pub trait Degrees: Sized {
    fn with_degrees(lat: f64, lon: f64) -> Self;
    fn with_secs(lat: f64, lon: f64) -> Self {
        Self::with_degrees(lat, lon)
    }
    fn with_dms<T: Into<Dms>>(lat: T, lon: T) -> Self {
        todo!()
    }
    fn degrees<T: From<LatLon>>(self) -> T;
    fn secs<T: From<LatLon>>(self) -> T {
        self.degrees()
    }
    fn dms(&self) -> (Dms, Dms) {
        todo!()
    }
}
pub struct LatLon<T = f64>(T, T);
impl LatLon {
    pub fn lat(&self) -> f64 {
        self.0
    }
    pub fn lon(&self) -> f64 {
        self.1
    }
}
impl From<LatLon> for (f64, f64) {
    fn from(value: LatLon) -> Self {
        todo!()
    }
}
// impl Degrees for LatLon {
//     fn with_degrees(lat: f64, lon: f64) -> Self {
//         todo!()
//     }
//     fn degrees(self) -> (f64, f64) {
//         todo!()
//     }
// }

pub struct Dms(i32, i32, f64);
impl From<(i32, i32, f64)> for Dms {
    fn from(value: (i32, i32, f64)) -> Self {
        todo!()
    }
}
impl From<Dms> for (i32, i32, f64) {
    fn from(value: Dms) -> Self {
        todo!()
    }
}

#[cfg(test)]
pub fn usage() {
    use geo::Point;
    use Degrees; // これが要る...

    // 真のベスト
    let (lat, lon) = Tokyo::with_degrees(1., 2.)
        .to_jgd2000()
        .to_jgd2011()
        .degrees();

    let (lat, lon) = Tokyo::with_degrees(1., 2.)
        .to_jgd2000()
        .to_jgd2011()
        .degrees();

    let (lat, lon) = Tokyo::with_dms((1, 2, 3.), (1, 2, 3.))
        .to_jgd2000()
        .to_jgd2011()
        .dms();
    let (d, m, s) = lat.into();

    // 使いにくい 分かりにくい
    // let jgd2000 = Tokyo::with_degrees(1., 2.)
    //     .to_jgd2000()
    //     .to_jgd2011()
    //     .degrees::<LatLon>();
    // jgd2000.lat();

    let (lat, lon) = Tokyo::with_secs(1., 2.).to_jgd2000().to_jgd2011().secs();

    let mut p = Point::new(1., 2.);
    let (lat, lon) = Tokyo::with_degrees(p.y(), p.x())
        .to_jgd2000()
        .to_jgd2011()
        .degrees();
    p = Point::new(lon, lat);
}
