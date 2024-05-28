use crate::{Degree, LatLon, XY};

/// 日本測地系 Tokyo Datum
pub struct TokyoDatum {
    pub lat: Degree,
    pub lon: Degree,
}
impl TokyoDatum {
    // Bessel楕円体

    pub fn with_xy(xy: impl Into<XY>) -> Self {
        let XY(x, y) = xy.into();
        Self { lat: y, lon: x }
    }

    /// `tky2jgd.par` を用いて [`JGD2000`] へ変換する
    pub fn to_jgd2000(self) -> JGD2000 {
        // 国土地理院時報(2002，97集)「世界測地系移行のための座標変換ソフトウェア”TKY2JGD"」
        // https://www.gsi.go.jp/common/000063173.pdf
        // > 地域毎の変換パラメータの格子点は，3 次メッシュの中央ではなく，南西隅に対応する。
        todo!()
    }

    /// 離島位置の補正量を用いて [`Tokyo97`] へ変換する
    pub fn to_tokyo97(self) {
        // 日本測地系における離島位置の補正量
        // https://www.jstage.jst.go.jp/article/sokuchi1954/49/3/49_3_181/_pdf
    }
}

/// 日本測地系 Tokyo97
pub struct Tokyo97<T>(T);

impl<T> Tokyo97<T> {
    /// 3パラメータを用いて [`JGD2000`] へ変換する
    pub fn to_jgd2000(self) {}

    /// 離島位置の補正量を用いて [`TokyoDatum`] へ逆変換する
    pub fn to_tokyo(self) {
        // 日本測地系における離島位置の補正量
        // https://www.jstage.jst.go.jp/article/sokuchi1954/49/3/49_3_181/_pdf
    }
}

/// Japanese Geodetic Datum 2000
///
/// 世界測地系 (JGD2000)
pub struct JGD2000 {
    pub lat: Degree,
    pub lon: Degree,
}

impl JGD2000 {
    // GRS80楕円体

    /// `touhokutaiheiyouoki2011.par` を用いて [`JGD2011`] へ変換する
    pub fn to_jgd2011(self) -> JGD2011 {
        // 地震時地殻変動に伴う座標値の変化を補正するソフトウェア“PatchJGD”
        // https://www.jstage.jst.go.jp/article/sokuchi/55/4/55_4_355/_pdf/-char/ja
        todo!()
    }

    /// `tky2jgd.par` を用いて [`TokyoDatum`] へ逆変換する
    pub fn to_tokyo(self) {}

    /// 3パラメータを用いて [`Tokyo97`] へ逆変換する
    pub fn to_tokyo97(self) {
        // https://www.drm.jp/jisx0410/JisGridSystem_1_Geoid.html

        // https://www.jstage.jst.go.jp/article/sokuchi1954/43/4/43_4_231/_pdf
        // > このパラメータによって定義される日本測地系がTokyo97である
    }
}

/// 世界測地系 JGD2011
pub struct JGD2011 {
    pub lat: Degree,
    pub lon: Degree,
}

impl JGD2011 {
    pub fn xy(self) -> [Degree; 2] {
        [self.lon, self.lat]
    }

    pub fn latlon(self) -> LatLon {
        todo!()
    }

    /// `touhokutaiheiyouoki2011.par` を用いて [`JGD2000`] へ逆変換する
    pub fn to_jgd2000(self) {}
}

// /// 平面直角座標系
// https://vldb.gsi.go.jp/sokuchi/surveycalc/surveycalc/algorithm/xy2bl/xy2bl.htm
// https://sw1227.hatenablog.com/entry/2018/11/30/200702
// pub struct PlaneRectangular<T>(T);

// /// Webメルカトル座標系
// pub struct WebMercator<T>(T);
