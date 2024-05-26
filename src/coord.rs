/// 度
pub type Degree = f64;

pub type XY = (Degree, Degree);

pub struct LatLon(pub Degree, pub Degree);
impl From<LatLon> for XY {
    fn from(bl: LatLon) -> Self {
        todo!()
    }
}

pub struct ECEF {}
impl ECEF {
    pub fn to_geodetic() {}
}
