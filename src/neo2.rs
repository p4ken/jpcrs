pub mod jgd {
    use super::{LatLon, Tokyo};

    pub fn from_tokyo(degrees: LatLon) -> Tokyo {
        Tokyo::from_degrees(degrees)
    }
}

pub struct Tokyo {
    degrees: LatLon,
}
impl Tokyo {
    pub fn new(degrees: LatLon) -> Self {
        Self { degrees }
    }
    pub fn from_degrees(degrees: LatLon) -> Self {
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
    pub fn into_inner(&self) -> LatLon {
        self.degrees
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LatLon(f64, f64);
impl LatLon {
    pub fn from_secs(_lat: f64, _lon: f64) -> Self {
        todo!()
    }
    pub fn as_tokyo(self) -> Tokyo {
        Tokyo::from_degrees(self)
    }
    pub fn to_secs(self) -> (f64, f64) {
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

    // Tokyoへの変換に見える
    // let LatLon(lat, lon) = Tokyo::from_degrees(LatLon(1., 2.))
    //     .to_jgd2000()
    //     .as_degrees();

    // 硬派
    let LatLon(lat, lon) = Tokyo::new(LatLon(1., 2.)).to_jgd2000().into_inner();

    let (lat, lon) = Tokyo::new(LatLon::from_secs(1., 2.))
        .to_jgd2000()
        .into_inner()
        .to_secs();

    // 標準ライブラリと比べて統一感がない
    let LatLon(lat, lon) = jgd::from_tokyo(LatLon(1., 2.)).to_jgd2000().degrees();

    let (lat, lon) = jgd::from_tokyo(LatLon::from_secs(1., 2.))
        .to_jgd2000()
        .degrees()
        .to_secs();

    // v0.1
    // APIが2系統あるのが微妙
    let (lat, lon) = crate::from_tokyo(1., 2.).to_jgd2000().degrees().into();

    // LatLon じゃなくなり LatLon に戻す...難解
    let LatLon(lat, lon) = LatLon(1., 2.).as_tokyo().to_jgd2000().degrees();

    let (lat, lon) = LatLon::from_secs(1., 2.)
        .as_tokyo()
        .to_jgd2000()
        .degrees()
        .to_secs();

    // 単位ミス！
    // let LatLon(lat, lon) = Tokyo(LatLon::from_seconds(1., 2.)).to_jgd2000().0;

    // 入力が冗長
    // let LatLon(lat, lon) = Tokyo {
    //     degrees: LatLon(1., 2.),
    // }
    // .to_jgd2000()
    // .degrees;
}
