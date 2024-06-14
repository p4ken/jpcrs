use std::ops::{Add, Mul, Sub};

/// 測地座標。
/// Geodetic coordinate.
// pub trait Geodetic: Sized {
//     fn with_lat_lon(lat_lon: (f64, f64)) -> Self;
//     fn lat_lon(&self) -> (f64, f64);
//     fn with_lon_lat(lon_lat: (f64, f64)) -> Self {
//         let (lon, lat) = lon_lat;
//         Self::with_lat_lon((lat, lon))
//     }
//     fn lon_lat(&self) -> (f64, f64) {
//         let (lat, lon) = self.lat_lon();
//         (lon, lat)
//     }
// }

// mod seal {
//     pub trait Sealed {}
//     impl Sealed for super::LatLon {}
//     impl Sealed for super::LonLat {}
// }

/// 緯度経度。
/// Latitude and longitude.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LatLon(pub f64, pub f64);
impl LatLon {
    /// 秒から度に変換する。
    /// Converts seconds to degrees.
    pub fn from_secs<T: Into<f64>>(lat: T, lon: T) -> Self {
        let [lat, lon] = [lat, lon].map(|x| x.into() / 3_600.);
        Self(lat, lon)
    }
    /// ミリ秒から度に変換する。
    /// Converts milliseconds to degrees.
    pub fn from_milli_secs<T: Into<f64>>(lat: T, lon: T) -> Self {
        Self::from_secs(lat, lon) * 0.001
    }
    /// マイクロ秒から度に変換する。
    /// Converts microseconds to degrees.
    pub fn from_micro_secs<T: Into<f64>>(lat: T, lon: T) -> Self {
        Self::from_milli_secs(lat, lon) * 0.001
    }
    // pub fn new(lat_lon: impl Into<(f64, f64)>) -> Self {
    //     let (lat, lon) = lat_lon.into();
    //     Self(lat, lon)
    // }
    // pub fn new_rev(lon_lat: impl Into<(f64, f64)>) -> Self {
    //     let (lon, lat) = lon_lat.into();
    //     Self(lat, lon)
    // }
    pub fn lat(&self) -> f64 {
        self.0
    }
    pub fn lon(&self) -> f64 {
        self.1
    }
    // pub fn rev(&self) -> (f64, f64) {
    //     (self.lon(), self.lat())
    // }
    pub fn abs(self) -> Self {
        self.map(f64::abs)
    }
    pub fn map(mut self, f: fn(f64) -> f64) -> Self {
        self.0 = f(self.0);
        self.1 = f(self.1);
        self
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
impl From<LatLon> for (f64, f64) {
    fn from(LatLon(lat, lon): LatLon) -> Self {
        (lat, lon)
    }
}

pub struct ECEF {}
impl ECEF {
    pub fn to_geodetic() {}
}
