use crate::LatLon;

#[cfg(feature = "tky2jgd")]
use crate::TKY2JGD;

pub fn from_tokyo(lat: f64, lon: f64) -> Tokyo {
    Tokyo::new(LatLon(lat, lon))
}

// pub fn from_tokyo_(p: impl Into<LatLon>) -> Tokyo {
//     Tokyo(p.into())
// }

/// 旧日本測地系。
/// Tokyo Datum, the older Japanese Datum.
///
/// EPSG: 4301
pub struct Tokyo {
    lat_lon: LatLon,
}
impl Tokyo {
    // Bessel楕円体

    fn new(lat_lon: impl Into<LatLon>) -> Self {
        let lat_lon = lat_lon.into();
        Self { lat_lon }
    }

    // pub fn with_lon_lat(degree: impl Into<LonLat>) -> Self {
    //     Self::new(degree.into())
    // }

    // pub fn with_xy(xy: impl Into<XY>) -> Self {
    //     let XY(x, y) = xy.into();
    //     Self { lat: y, lon: x }
    // }

    /// [`TKY2JGD`] を用いて [`JGD2000`] へ変換する。
    ///
    /// パラメータグリッドの範囲外の場合は、3パラメータにフォールバックする。
    #[cfg(feature = "tky2jgd")]
    pub fn to_jgd2000(&self) -> Jgd2000 {
        // 国土地理院時報(2002，97集)「世界測地系移行のための座標変換ソフトウェア”TKY2JGD"」
        // https://www.gsi.go.jp/common/000063173.pdf
        // > 地域毎の変換パラメータの格子点は，3 次メッシュの中央ではなく，南西隅に対応する。
        match TKY2JGD.interpolate(self.lat_lon) {
            Some(shift) => Jgd2000::new(self.lat_lon + shift),
            None => self.to_tokyo97().to_jgd2000(),
        }
    }

    /// 離島位置の補正量を用いて [`Tokyo97`] へ変換する
    pub fn to_tokyo97(&self) -> Tokyo97 {
        // 日本測地系における離島位置の補正量
        // https://www.jstage.jst.go.jp/article/sokuchi1954/49/3/49_3_181/_pdf
        todo!()
    }
}

/// Tokyo97 座標系。
/// Tokyo97, the older Japanese Datum.
///
/// 3パラメータを用いた変換式 ([飛田幹男, 1997](crate#references)) で定義される旧日本測地系。
pub struct Tokyo97 {
    lat_lon: LatLon,
}
impl Tokyo97 {
    /// 3パラメータを用いて [`JGD2000`] へ変換する
    pub fn to_jgd2000(&self) -> Jgd2000 {
        todo!()
    }

    /// 離島位置の補正量を用いて [`TokyoDatum`] へ逆変換する
    pub fn to_tokyo(&self) {
        // 日本測地系における離島位置の補正量
        // https://www.jstage.jst.go.jp/article/sokuchi1954/49/3/49_3_181/_pdf
    }
}

/// 世界測地系。
/// JGD2000, Japanese Geodetic Datum 2000.
///
/// EPSG: 4612
pub struct Jgd2000 {
    lat_lon: LatLon,
}

impl Jgd2000 {
    // GRS80楕円体

    pub fn new(degree: impl Into<LatLon>) -> Self {
        let lat_lon = degree.into();
        Self { lat_lon }
    }

    /// `touhokutaiheiyouoki2011.par` を用いて [`JGD2011`] へ変換する
    pub fn to_jgd2011(&self) -> Jgd2011 {
        // 地震時地殻変動に伴う座標値の変化を補正するソフトウェア“PatchJGD”
        // https://www.jstage.jst.go.jp/article/sokuchi/55/4/55_4_355/_pdf/-char/ja
        todo!()
    }

    /// `tky2jgd.par` を用いて [`TokyoDatum`] へ逆変換する
    pub fn to_tokyo(&self) {}

    /// 3パラメータを用いて [`Tokyo97`] へ逆変換する
    pub fn to_tokyo97(&self) {
        // https://www.drm.jp/jisx0410/JisGridSystem_1_Geoid.html

        // https://www.jstage.jst.go.jp/article/sokuchi1954/43/4/43_4_231/_pdf
        // > このパラメータによって定義される日本測地系がTokyo97である
    }
}

/// 世界測地系。
/// JGD2011, Japanese Geodetic Datum 2011.
///
/// EPSG: 6668
pub struct Jgd2011 {
    lat_lon: LatLon,
}
impl Jgd2011 {
    pub fn lat_lon(&self) -> (f64, f64) {
        self.lat_lon.into()
    }

    pub fn lon_lat(&self) -> (f64, f64) {
        self.lat_lon.rev()
    }

    /// `touhokutaiheiyouoki2011.par` を用いて [`JGD2000`] へ逆変換する
    pub fn to_jgd2000(&self) {}
}

// /// 平面直角座標系
// https://vldb.gsi.go.jp/sokuchi/surveycalc/surveycalc/algorithm/xy2bl/xy2bl.htm
// https://sw1227.hatenablog.com/entry/2018/11/30/200702
// pub struct PlaneRectangular<T>(T);

// /// Webメルカトル座標系
// pub struct WebMercator<T>(T);
