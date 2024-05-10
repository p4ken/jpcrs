use std::ops::Mul;

/// Latitude and longitude in degrees.
pub struct Degree(Geodetic<f64>); // 0.14ns (f32: 77ms)
impl From<MilliSecond> for Degree {
    fn from(ms: MilliSecond) -> Self {
        todo!()
    }
}
impl From<NanoSecond> for Degree {
    fn from(value: NanoSecond) -> Self {
        todo!()
    }
}

// pub struct MicroDegree(Geodetic<i32>); // 8% 22cm

/// Latitude and longitude in milli seconds.
pub struct MilliSecond(pub(crate) Geodetic<i32>); // 30% 60mm

// pub struct MicroSecond(Geodetic<i64>); // 0% 0.06mm

/// Latitude and longitude in nano seconds.
pub struct NanoSecond(pub(crate) Geodetic<i64>); // 0.007% 0mm
pub struct NanoSecond_ {
    pub lat: i64,
    pub lon: i64,
}

// pub struct Degree {
//     int: i16,
//     nanos: i32,
// }

// NanoSecondの方がよい
// pub struct Second {
//     second: i32, // 0.03%
//     micro: i32,  // 0.04%
// }

/// Latitude and longitude.
#[derive(Debug)]
pub struct Geodetic<T> {
    /// Latitude
    pub lat: T,

    /// Longitude
    pub lon: T,
}
// impl<T: Mul<Output = T> + Copy> Mul<T> for Geodetic<T> {
//     type Output = Self;
//     fn mul(self, rhs: T) -> Self::Output {
//         Self {
//             lat: self.lat * rhs,
//             lon: self.lon * rhs,
//         }
//     }
// }
#[cfg(feature = "geo-types")]
impl From<geo_types::Coord> for Geodetic<Degree> {
    fn from(coord: geo_types::Coord) -> Self {
        geo_types::Point::new(1.0, 1.0);
        todo!()
    }
}

/// Latitude and longitude.
#[derive(Debug)]
pub struct LatLon {
    lat_us: i64,
    lon_us: i64,
}
impl LatLon {
    fn from_millis(lat_ms: i32, lon_ms: i32) {}
    fn millis(&self) -> (i32, i32) {
        todo!()
    }
    fn from_degrees(lat_d: f64, lon_d: f64) {}
    fn degrees(&self) -> (f64, f64) {
        todo!()
    }
}

pub struct ECEF {}
impl ECEF {
    pub fn to_geodetic() {}
}
