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

/// 度単位の緯度と経度のペア。
/// Latitude and longitude in degrees.
#[derive(Debug, Clone, Copy)]
pub struct LatLon(pub f64, pub f64);
impl LatLon {
    pub fn new(lat_lon: impl Into<(f64, f64)>) -> Self {
        let (lat, lon) = lat_lon.into();
        Self(lat, lon)
    }
    pub fn new_rev(lon_lat: impl Into<(f64, f64)>) -> Self {
        let (lon, lat) = lon_lat.into();
        Self(lat, lon)
    }
    pub fn lat(&self) -> f64 {
        self.0
    }
    pub fn lon(&self) -> f64 {
        self.1
    }
    pub fn rev(&self) -> (f64, f64) {
        (self.lon(), self.lat())
    }
    // // これだと取り出す時に再度逆転させる必要がある。
    // pub fn from_lonlat(lonlat: impl Into<(f64, f64)>) -> Self {
    //     let (lon, lat) = lonlat.into();
    //     Self(lat, lon)
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
impl From<LatLon> for (f64, f64) {
    fn from(value: LatLon) -> Self {
        todo!()
    }
}

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
// impl From<LatLon> for LonLat {
//     fn from(LatLon(lat, lon): LatLon) -> Self {
//         Self(lon, lat)
//     }
// }
// impl<T: Into<(f64, f64)>> From<T> for LonLat {
//     fn from(lonlat: T) -> Self {
//         let (lon, lat) = lonlat.into();
//         Self(lon, lat)
//     }
// }

pub struct ECEF {}
impl ECEF {
    pub fn to_geodetic() {}
}
