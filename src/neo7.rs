#![allow(unused)]

pub struct Tokyo;
impl Tokyo {
    pub fn new(coord: impl Into<LatLon>) -> Self {
        Self
    }
}

pub struct LatLon<L: Into<Degrees> = f64>(L, L);
pub struct Degrees(f64);
impl From<f64> for Degrees {
    fn from(value: f64) -> Self {
        todo!()
    }
}
pub struct Seconds(f64);

fn usage() {
    Tokyo::new(LatLon(1., 2.));
    Tokyo::new(LatLon::<Seconds>(1., 2.));
}
