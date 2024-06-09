use std::ops::{Add, Mul, Sub};

/// 測地座標。
/// Geodetic coordinate.
pub trait Geodetic: Copy + From<LatLon> + Into<LatLon> {
    // fn from_degrees(_: f64, _: f64) -> Self;
    /// 度単位の値のペア。
    fn degrees(&self) -> (f64, f64);
    // fn to_latlon(&self) -> LatLon;
}

// mod seal {
//     pub trait Sealed {}
//     impl Sealed for super::LatLon {}
//     impl Sealed for super::LonLat {}
// }

/// 度単位の緯度と経度のペア。
/// Latitude and longitude in degrees.
#[derive(Debug, Clone, Copy)]
pub struct LatLon(pub f64, pub f64);
impl LatLon {
    // pub fn new(lat: f64, lon: f64) -> Self {
    //     Self(lat, lon)
    // }
    pub fn lat(&self) -> f64 {
        self.0
    }
    pub fn lon(&self) -> f64 {
        self.1
    }
    // // これだと取り出す時に再度逆転させる必要がある。
    // pub fn from_lonlat(lonlat: impl Into<(f64, f64)>) -> Self {
    //     let (lon, lat) = lonlat.into();
    //     Self(lat, lon)
    // }
}
impl Geodetic for LatLon {
    fn degrees(&self) -> (f64, f64) {
        (self.0, self.1)
    }
    // fn to_latlon(&self) -> LatLon {
    //     self.clone()
    // }
}
impl From<LonLat> for LatLon {
    fn from(LonLat(lon, lat): LonLat) -> Self {
        Self(lat, lon)
    }
}
impl Add<LatLon> for LatLon {
    type Output = Self;
    fn add(self, rhs: LatLon) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<LatLon> for LatLon {
    type Output = Self;
    fn sub(self, rhs: LatLon) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Mul<f64> for LatLon {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
// impl From<LatLon> for (f64, f64) {
//     fn from(LatLon(lat, lon): LatLon) -> Self {
//         (lat, lon)
//     }
// }

/// 度単位の経度と緯度のペア。
/// Longitude and latitude in degrees.
#[derive(Debug, Clone, Copy)]
pub struct LonLat(f64, f64);
impl LonLat {
    pub fn new(lonlat: impl Into<(f64, f64)>) -> Self {
        let (lon, lat) = lonlat.into();
        Self(lon, lat)
    }
}
impl Geodetic for LonLat {
    fn degrees(&self) -> (f64, f64) {
        (self.0, self.1)
    }
    // fn to_latlon(&self) -> LatLon {
    //     let (lon, lat) = self.0;
    //     LatLon(lat, lon)
    // }
}
impl From<LatLon> for LonLat {
    fn from(LatLon(lat, lon): LatLon) -> Self {
        Self(lon, lat)
    }
}
// impl From<LonLat> for (f64, f64) {
//     fn from(LonLat((lon, lat)): LonLat) -> Self {
//         (lat, lon)
//     }
// }
// impl<T: Into<(f64, f64)>> From<T> for LonLat {
//     fn from(lonlat: T) -> Self {
//         let (lon, lat) = lonlat.into();
//         Self((lon, lat))
//     }
// }

pub struct ECEF {}
impl ECEF {
    pub fn to_geodetic() {}
}
