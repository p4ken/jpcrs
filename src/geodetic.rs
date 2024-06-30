use crate::LatLon;

pub trait Degrees: Sized {
    fn with_degrees<D: Into<f64>>(lat: D, lon: D) -> Self;
    fn degrees(&self) -> (f64, f64);
}
impl<T: From<LatLon> + AsRef<LatLon>> Degrees for T {
    fn with_degrees<D: Into<f64>>(lat: D, lon: D) -> Self {
        LatLon::new(lat, lon).into()
    }

    fn degrees(&self) -> (f64, f64) {
        let ll = self.as_ref();
        (ll.lat(), ll.lon())
    }
}
