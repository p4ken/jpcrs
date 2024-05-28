/// 度
pub type Degree = f64;

pub struct XY(pub Degree, pub Degree);
impl From<LatLon> for XY {
    fn from(latlon: LatLon) -> Self {
        let LatLon(lat, lon) = latlon;
        Self(lon, lat)
    }
}
impl<T: Into<[f64; 2]>> From<T> for XY {
    fn from(xy: T) -> Self {
        let [x, y] = xy.into();
        Self(x, y)
    }
}

pub struct LatLon(pub Degree, pub Degree);

pub struct ECEF {}
impl ECEF {
    pub fn to_geodetic() {}
}
