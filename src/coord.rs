use std::ops::{Add, Mul, Sub};

/// 度単位の緯度経度。
/// Latitude and longitude in degrees.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LatLon(pub f64, pub f64);
impl LatLon {
    /// 度分秒から度に変換する。
    /// Convertes degrees, minutes, seconds to decimal degrees.
    ///
    /// # Examples
    ///
    /// 日本緯経度原点: 東経 139°44′28.8869” 北緯　35°39′29.1572”
    ///
    /// ```
    /// use jgd::LatLon;
    ///
    /// let origin = LatLon::from_dms((35, 39, 29.1572), (139, 44, 28.8869));
    /// ```
    pub fn from_dms<T: Into<Dms>>(lat: T, lon: T) -> Self {
        Self(lat.into().to_degrees(), lon.into().to_degrees())
    }

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

/// 度分秒。
/// Degrees, minutes, seconds.
///
/// # Examples
///
/// ```
/// use jgd::Dms;
///
/// let akashi = Dms(135, 0, 0.0);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Dms(pub i32, pub i32, pub f64);
impl Dms {
    fn to_degrees(&self) -> f64 {
        f64::from(self.0) + f64::from(self.1) / 60. + self.2 / 3_600.
    }
}
impl<D: Into<i32>, M: Into<i32>, S: Into<f64>> From<(D, M, S)> for Dms {
    fn from((d, m, s): (D, M, S)) -> Self {
        Self(d.into(), m.into(), s.into())
    }
}

/// 三次元直交座標。
#[derive(Debug, Clone, Copy)]
pub struct ECEF {
    x: f64,
    y: f64,
    z: f64,
}
impl ECEF {
    /// コンストラクタ。
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
