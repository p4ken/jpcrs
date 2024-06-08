use crate::neo::Degree;

pub trait Geodetic<T> {
    fn lat(&self) -> T;
}

pub struct LatLon<T>(pub T, pub T);
impl Geodetic<Degree> for LatLon<f64> {
    fn lat(&self) -> Degree {
        self.0.into()
    }
}
impl<T: Copy> Geodetic<T> for LatLon<T> {
    fn lat(&self) -> T {
        self.0
    }
}

pub struct LonLat<T>(pub T, pub T);
impl Geodetic<Degree> for LonLat<f64> {
    fn lat(&self) -> Degree {
        self.0.into()
    }
}
impl<T: Copy> Geodetic<T> for LonLat<T> {
    fn lat(&self) -> T {
        self.1
    }
}
impl<T: Copy> LonLat<T> {
    pub fn lonlat(&self) -> (T, T) {
        (self.0, self.1)
    }
}
impl LonLat<Degree> {
    pub fn degree(&self) -> (f64, f64) {
        (self.0.degree(), self.1.degree())
    }
}
impl<T> From<(T, T)> for LonLat<T> {
    fn from((lon, lat): (T, T)) -> Self {
        Self(lon, lat)
    }
}
// impl<T, U: Into<(T, T)>> From<U> for LonLat<T> {
//     fn from(lonlat: U) -> Self {
//         let (lon, lat) = lonlat.into();
//         Self(lon, lat)
//     }
// }
