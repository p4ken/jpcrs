pub fn from_tokyo(degrees: LatLon) -> Tokyo {
    Tokyo::new(degrees)
}

pub struct Tokyo {
    degrees: LatLon,
}
impl Tokyo {
    // pub fn new(_degrees: impl Into<LatLon>) -> Self {
    pub fn new(degrees: LatLon) -> Self {
        Self { degrees }
    }
    pub fn to_jgd2000(&self) -> Jgd2000 {
        Jgd2000 {
            degrees: self.degrees,
        }
    }
}

pub struct Jgd2000 {
    degrees: LatLon,
}
impl Jgd2000 {
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LatLon(f64, f64);
impl LatLon {
    pub fn from_seconds(_lat: f64, _lon: f64) -> Self {
        todo!()
    }
    pub fn to_seconds(&self) -> (f64, f64) {
        (self.0, self.1)
    }
}
// impl From<(f64, f64)> for LatLon {
//     fn from(_: (f64, f64)) -> Self {
//         todo!()
//     }
// }
// impl From<LatLon> for (f64, f64) {
//     fn from(_: LatLon) -> Self {
//         todo!()
//     }
// }

#[allow(unused_variables)]
fn _usage() {
    // 順序ミス！
    // let (lon, lat) = Tokyo::new((1., 2.)).to_jgd2000().degrees().into();

    let LatLon(lat, lon) = from_tokyo(LatLon(1., 2.)).to_jgd2000().degrees();

    // Tokyoへの変換に見える
    // let LatLon(lat, lon) = Tokyo::new(LatLon(1., 2.)).to_jgd2000().degrees();

    let (lat, lon) = Tokyo::new(LatLon::from_seconds(1., 2.))
        .to_jgd2000()
        .degrees()
        .to_seconds();

    // 入力が冗長
    // let LatLon(lat, lon) = Tokyo {
    //     degrees: LatLon(1., 2.),
    // }
    // .to_jgd2000()
    // .degrees;

    // 単位ミス！
    // let LatLon(lat, lon) = Tokyo(LatLon::from_seconds(1., 2.)).to_jgd2000().0;
}
