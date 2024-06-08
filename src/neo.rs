/// 度単位。
#[derive(Debug, Clone, Copy)]
pub struct Degree(f64);
impl Degree {
    pub fn degree(self) -> f64 {
        self.0
    }
}
impl From<f64> for Degree {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

#[deprecated = "experimental"]
pub struct DMS;

// pub struct MicroSecond(pub i32);

#[derive(Debug, Clone, Copy)]
pub struct LatLon<T: Into<Degree> = Degree> {
    pub lat: T,
    pub lon: T,
}
impl LatLon<Degree> {
    pub fn new<T: Into<Degree>>(lat: T, lon: T) -> Self {
        Self {
            lat: lat.into(),
            lon: lon.into(),
        }
    }
}
// impl<T: Into<Degree>> From<LatLon<T>> for LatLon<Degree> {
//     fn from(value: LatLon<T>) -> Self {
//         Self::new(value.lat, value.lon)
//     }
// }

fn draft() {
    let bl = LatLon::new(1.0, 2.0);
    let bl = LatLon { lat: 1.0, lon: 2.0 };
}
