use std::marker::PhantomData;

pub struct LatLon<T> {
    lat: f64,
    lon: f64,
    crs: PhantomData<T>,
}
impl<T> LatLon<T> {
    pub fn from_degrees(lat: f64, lon: f64) -> Self {
        let crs = PhantomData::<T>;
        Self { lat, lon, crs }
    }
    pub fn degrees(&self) -> (f64, f64) {
        (self.lat, self.lon)
    }
}

pub struct Tokyo;
pub struct Jgd2000;

pub fn from_tokyo(lat: f64, lon: f64) -> LatLon<Tokyo> {
    LatLon::from_degrees(lat, lon)
}

impl LatLon<Tokyo> {
    pub fn to_jgd2000(&self) -> LatLon<Jgd2000> {
        todo!()
    }
}

#[allow(dead_code)]
fn usage() {
    let tokyo = LatLon::<Tokyo>::from_degrees(1., 2.);
    let (_lat, _lon) = tokyo.to_jgd2000().degrees();
}
