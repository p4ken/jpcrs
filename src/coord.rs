use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum DegreeError {
    Latitude,
    Longitude,
}
impl Display for DegreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DegreeError::Latitude => f.write_str("invalid latitude in degrees"),
            DegreeError::Longitude => f.write_str("invalid longitude in degrees"),
        }
    }
}
impl Error for DegreeError {}

/// 度単位の座標。
/// Coordinate in degrees.
#[derive(Debug, Clone, Copy)]
pub struct Degree {
    lat: f64,
    lon: f64,
}
impl Degree {
    /// XY座標で初期化する。
    ///
    /// # Panics
    ///
    /// It panics if `xy` is out of latitude and longitude in degrees.
    pub fn new(xy: impl Into<[f64; 2]>) -> Self {
        Self::try_with_xy(xy.into()).unwrap()
    }
    pub fn try_with_xy(xy: [f64; 2]) -> Result<Self, DegreeError> {
        let [lon, lat] = xy;
        if lat.abs() > 90. {
            return Err(DegreeError::Latitude);
        }
        if lon.abs() > 180. {
            return Err(DegreeError::Longitude);
        }
        Ok(Self::new_unchecked(lat, lon))
    }
    pub(crate) fn new_unchecked(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }
    // pub fn with_xy(xy: [f64; 2]) -> Self {
    //     Self::new(xy)
    // }
    pub fn try_with_latlon(latlon: [f64; 2]) -> Result<Self, DegreeError> {
        let [lat, lon] = latlon;
        Self::try_with_xy([lon, lat])
    }
    pub fn xy(self) -> [f64; 2] {
        [self.lon, self.lat]
    }
    pub fn latlon(self) -> [f64; 2] {
        [self.lat, self.lon]
    }
}

// pub struct XY<T>(T, T);
// impl<T, U: Into<[T; 2]>> From<U> for XY<T> {
//     fn from(into_xy: U) -> Self {
//         let [x, y] = into_xy.into();
//         Self(x, y)
//     }
// }
// pub struct LatLon<T>(T, T);

// pub struct XY(pub Degree, pub Degree);
// impl From<LatLon> for XY {
//     fn from(latlon: LatLon) -> Self {
//         let LatLon(lat, lon) = latlon;
//         Self(lon, lat)
//     }
// }
// impl<T: Into<[f64; 2]>> From<T> for XY {
//     fn from(xy: T) -> Self {
//         let [x, y] = xy.into();
//         Self(x, y)
//     }
// }

// pub struct LatLon(pub Degree, pub Degree);

pub struct ECEF {}
impl ECEF {
    pub fn to_geodetic() {}
}
