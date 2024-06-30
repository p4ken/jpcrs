#![allow(unused)]

pub const SECONDS: i32 = 1;
pub const DEGREES: i32 = SECONDS * 3_600;

pub struct LatLon<const UNIT: i32 = DEGREES>(f64, f64);
pub struct Tokyo<const UNIT: i32>(LatLon<UNIT>);
impl<const UNIT: i32> Tokyo<UNIT> {
    pub fn to_jgd2000(self) -> Jgd2000<UNIT> {
        Jgd2000(self.0)
    }
}
pub struct Jgd2000<const UNIT: i32>(LatLon<UNIT>);

fn usage() {
    Tokyo(LatLon(1., 2.)).to_jgd2000().0;
    Tokyo(LatLon::<SECONDS>(1., 2.)).to_jgd2000().0;
}
