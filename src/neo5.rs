#![allow(unused)]

pub mod jgd {
    pub struct Tokyo;
    impl Tokyo {
        pub fn new(lat: f64, lon: f64) -> Self {
            Self
        }
        pub fn to_jgd2000(self) -> Jgd2000 {
            Jgd2000
        }
    }
    impl Geodetic for Tokyo {
        fn with_degrees(lat: f64, lon: f64) -> Self {
            Self::new(lat, lon)
        }
        fn degrees(&self) -> (f64, f64) {
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
    impl Geodetic for Jgd2011 {
        fn with_degrees(lat: f64, lon: f64) -> Self {
            Self
        }
        fn degrees(&self) -> (f64, f64) {
            (0., 0.)
        }
    }

    // TODO sealed
    pub trait Geodetic: Sized {
        fn with_degrees(lat: f64, lon: f64) -> Self;
        fn with_secs(lat: f64, lon: f64) -> Self {
            Self::with_degrees(lat, lon)
        }
        fn with_dms<T: Into<Dms>>(lat: T, lon: T) -> Self {
            todo!()
        }

        fn degrees(&self) -> (f64, f64);
        fn secs(&self) -> (f64, f64) {
            self.degrees()
        }
        fn dms(&self) -> (Dms, Dms) {
            todo!()
        }
    }

    pub struct Dms {
        d: i32,
        m: i32,
        s: f64,
    }

    impl Dms {
        pub fn d(&self) -> i32 {
            self.d
        }

        pub fn m(&self) -> i32 {
            self.m
        }

        pub fn s(&self) -> f64 {
            self.s
        }
    }
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
}

#[cfg(test)]
pub fn usage() {
    use geo::Point;
    use jgd::{Geodetic, Tokyo};

    // 真のベスト
    let (lat, lon) = Tokyo::new(1., 2.).to_jgd2000().to_jgd2011().into();

    let (lat, lon) = Tokyo::with_degrees(1., 2.)
        .to_jgd2000()
        .to_jgd2011()
        .degrees();

    let (lat, lon) = Tokyo::with_secs(1., 2.).to_jgd2000().to_jgd2011().secs();

    let (lat, lon) = Tokyo::with_dms((1, 2, 3.), (1, 2, 3.))
        .to_jgd2000()
        .to_jgd2011()
        .dms();
    let d = lat.d();
    let (d, m, s) = lat.into();

    let mut p = Point::new(1., 2.);
    let (y, x) = Tokyo::new(p.y(), p.x()).to_jgd2000().to_jgd2011().into();
    p = Point::new(x, y);
}
