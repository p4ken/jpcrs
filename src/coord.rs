pub struct Degree(f64);
pub struct MilliSecond(i32);
pub struct NanoSecond(i64);

#[derive(Debug, Clone, Copy)]
pub struct LatLon<T> {
    lat: T,
    lon: T,
}

impl<T> LatLon<T> {
    pub fn new(lat: T, lon: T) {}
}

impl LatLon<NanoSecond> {
    pub fn from_degree<T: Into<f64>>(lat: T, lon: T) {}
    pub fn from_milli_seconds<T: Into<i32>>(lat: T, lon: T) {}
    pub fn from_nano_seconds<T: Into<i64>>(lat: T, lon: T) {}

    // ミスを誘発するのでbad
    pub fn to_jgd2000() {}
}

// pub trait ToJgd2000 {
//     fn to_jgd2000();
// }
// impl ToJgd2000 for LatLon<NanoSecond> {
//     // ミスを誘発するのでbad
//     fn to_jgd2000() {}
// }

pub fn to_jgd2000() {}

// LatLonはf64 degree固定の代わりに
// par関連はus単位、離島はcm単位の変化量を返してあげる、とか
