pub mod jgd {
    use super::{LatLon, Tokyo};

    pub fn from_tokyo(degrees: LatLon) -> Tokyo {
        Tokyo::new(degrees)
    }
}

pub struct Tokyo {
    degrees: LatLon,
}
impl Tokyo {
    pub fn new(degrees: LatLon) -> Self {
        Self { degrees }
    }
    pub fn to_jgd2000(&self) -> Jgd2000 {
        Jgd2000 {
            degrees: self.degrees,
        }
    }
}
impl From<Tokyo> for LatLon {
    fn from(value: Tokyo) -> Self {
        value.degrees
    }
}
impl From<LatLon> for Tokyo {
    fn from(degrees: LatLon) -> Self {
        Self { degrees }
    }
}

pub struct Jgd2000 {
    degrees: LatLon,
}
impl Jgd2000 {
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
    pub fn to_degrees(&self) -> LatLon {
        self.degrees()
    }
    pub fn into_inner(&self) -> LatLon {
        self.degrees()
    }
}
impl AsRef<LatLon> for Jgd2000 {
    fn as_ref(&self) -> &LatLon {
        &self.degrees
    }
}
impl From<Jgd2000> for LatLon {
    fn from(value: Jgd2000) -> Self {
        value.degrees
    }
}
impl From<LatLon> for Jgd2000 {
    fn from(degrees: LatLon) -> Self {
        Self { degrees }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LatLon(f64, f64);
impl LatLon {
    pub fn from_secs(_lat: f64, _lon: f64) -> Self {
        todo!()
    }
    pub fn in_tokyo(self) -> Tokyo {
        Tokyo::new(self)
    }
    pub fn from_tokyo(self) -> Tokyo {
        self.in_tokyo()
    }
    pub fn transform_from_tokyo(self) -> Tokyo {
        self.in_tokyo()
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

pub trait Degrees: Sized {
    // TODO sealed
    // fn with_degrees(lat: f64, lon: f64) -> Self;
    fn with_secs(lat: f64, lon: f64) -> Self;
    // fn degrees(self) -> (f64, f64);
    fn secs(self) -> (f64, f64);
}
impl<T: From<LatLon> + Into<LatLon>> Degrees for T {
    // fn with_degrees(lat: f64, lon: f64) -> Self {
    //     LatLon(lat, lon).into()
    // }
    fn with_secs(lat: f64, lon: f64) -> Self {
        LatLon::from_secs(lat, lon).into()
    }
    // fn degrees(self) -> (f64, f64) {
    //     let LatLon(lat, lon) = self.into();
    //     (lat, lon)
    // }
    fn secs(self) -> (f64, f64) {
        self.into().to_secs()
    }
}

#[allow(unused_variables)]
#[cfg(feature = "default")]
mod tests {
    use super::{jgd, Degrees, LatLon, Tokyo};

    fn _usage() {
        // 順序ミス！
        // let (lon, lat) = Tokyo::new((1., 2.)).to_jgd2000().degrees().into();

        // 硬派
        let LatLon(lat, lon) = Tokyo::new(LatLon(1., 2.)).to_jgd2000().into_inner();

        // ベスト候補
        let LatLon(lat, lon) = Tokyo::new(LatLon(1., 2.)).to_jgd2000().into();
        // let (lat, lon) = Tokyo::with_degrees(1., 2.).to_jgd2000().degrees();
        let (lat, lon) = Tokyo::with_secs(1., 2.).to_jgd2000().secs();

        // 実装の重複が多い
        // let (lat, lon) = Tokyo::from_secs(1., 2.).to_jgd2000().secs();

        // 標準ライブラリと比べて統一感がない？
        let LatLon(lat, lon) = jgd::from_tokyo(LatLon(1., 2.)).to_jgd2000().to_degrees();
        let LatLon(lat, lon) = *jgd::from_tokyo(LatLon(1., 2.)).to_jgd2000().as_ref();
        let LatLon(lat, lon) = jgd::from_tokyo(LatLon(1., 2.)).to_jgd2000().into();

        let (lat, lon) = jgd::from_tokyo(LatLon::from_secs(1., 2.))
            .to_jgd2000()
            .secs();

        // v0.1
        // APIが2系統あるのが微妙
        let (lat, lon) = crate::from_tokyo(1., 2.).to_jgd2000().degrees().into();

        // LatLon じゃなくなり LatLon に戻す...難解
        let LatLon(lat, lon) = LatLon(1., 2.).in_tokyo().to_jgd2000().degrees();
        // let LatLon(lat, lon) = LatLon(1., 2.).from_tokyo().to_jgd2000().to_degrees();
        // let LatLon(lat, lon) = LatLon(1., 2.).transform_from_tokyo().to_jgd2000().degrees();

        let (lat, lon) = LatLon::from_secs(1., 2.)
            .in_tokyo()
            .to_jgd2000()
            .degrees() // secsが欲しいのに
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
}
