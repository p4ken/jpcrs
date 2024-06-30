use std::ops::{Add, Div, Mul, Sub};

/// 緯度経度。
/// Latitude and longitude of a coordinate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct LatLon {
    lat: f64,
    lon: f64,
}
impl LatLon {
    pub(crate) fn new<T: Into<f64>>(lat: T, lon: T) -> Self {
        let (lat, lon) = (lat.into(), lon.into());
        Self { lat, lon }
    }

    /// 度分秒から変換する。
    /// Converts from degrees, minutes, seconds.
    ///
    /// # Examples
    ///
    /// 東経 139°44′28.8869” 北緯 35°39′29.1572” (日本緯経度原点)
    ///
    /// ```
    /// use jgd::LatLon;
    ///
    /// let origin = LatLon::from_dms((35, 39, 29.1572), (139, 44, 28.8869));
    /// ```
    pub fn from_dms<T: Into<Dms>>(lat: T, lon: T) -> Self {
        Self::new(lat.into().to_degrees(), lon.into().to_degrees())
    }

    /// 秒から変換する。
    /// Converts from seconds.
    pub fn from_secs<T: Into<f64>>(lat: T, lon: T) -> Self {
        Self::new(lat.into(), lon.into()) / 3_600.
    }

    /// ミリ秒から度に変換する。
    /// Converts from milliseconds.
    pub fn from_milli_secs<T: Into<f64>>(lat: T, lon: T) -> Self {
        Self::from_secs(lat, lon) / 1_000.
    }

    /// マイクロ秒から度に変換する。
    /// Converts from microseconds.
    pub fn from_micro_secs<T: Into<f64>>(lat: T, lon: T) -> Self {
        Self::from_milli_secs(lat, lon) / 1_000.
    }

    /// 度分秒に変換する。
    /// Converts to degrees, minutes, seconds.
    pub fn to_dms(&self) -> (Dms, Dms) {
        [self.lat, self.lon].map(Dms::from_degrees).into()
    }

    pub(crate) fn lat(&self) -> f64 {
        self.lat
    }

    pub(crate) fn lon(&self) -> f64 {
        self.lon
    }

    fn map(mut self, f: impl Fn(f64) -> f64) -> Self {
        self.lat = f(self.lat);
        self.lon = f(self.lon);
        self
    }
}
impl Add<LatLon> for LatLon {
    type Output = Self;
    fn add(mut self, rhs: LatLon) -> Self::Output {
        self.lat += rhs.lat;
        self.lon += rhs.lon;
        self
    }
}
impl Sub<LatLon> for LatLon {
    type Output = Self;
    fn sub(mut self, rhs: LatLon) -> Self::Output {
        self.lat -= rhs.lat;
        self.lon -= rhs.lon;
        self
    }
}
impl Mul<f64> for LatLon {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|x| x * rhs)
    }
}
impl Div<f64> for LatLon {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self.map(|x| x / rhs)
    }
}
impl AsRef<LatLon> for LatLon {
    fn as_ref(&self) -> &LatLon {
        self
    }
}

/// 度分秒。
/// Degrees, minutes, seconds.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Dms {
    /// 度。
    /// Degrees.
    pub d: i32,

    /// 分。
    /// Minutes.
    pub m: i32,

    /// 秒。
    /// Seconds.
    pub s: f64,
}
impl Dms {
    fn new<D: Into<i32>, M: Into<i32>, S: Into<f64>>(d: D, m: M, s: S) -> Self {
        let d = d.into();
        let m = m.into();
        let s = s.into();
        Self { d, m, s }
    }
    fn from_degrees(deg: f64) -> Self {
        let d = deg as i32;
        let m = (deg * 60. % 60.) as i32;
        let s = (deg * 3600.) % 60.;
        Self::new(d, m, s)
    }
    fn to_degrees(&self) -> f64 {
        f64::from(self.d) + f64::from(self.m) / 60. + self.s / 3_600.
    }
}
impl<D: Into<i32>, M: Into<i32>, S: Into<f64>> From<(D, M, S)> for Dms {
    fn from((d, m, s): (D, M, S)) -> Self {
        Self::new(d, m, s)
    }
}
// impl From<Dms> for (i32, i32, f64) {
//     fn from(Dms { d, m, s }: Dms) -> Self {
//         (d, m, s)
//     }
// }

/// 三次元直交座標。
/// Earth-centered, Earth-fixed coordinate.
#[derive(Debug, Clone, Copy)]
pub struct ECEF {
    x: f64,
    y: f64,
    z: f64,
}
impl ECEF {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}
impl Add for ECEF {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for ECEF {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
