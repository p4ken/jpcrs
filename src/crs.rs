use crate::Degree;

/// 旧日本測地系。
/// Tokyo Datum, the older Japanese Datum.
///
/// EPSG: 4301
pub struct Tokyo {
    degree: Degree,
}
impl Tokyo {
    // Bessel楕円体

    // pub fn with_xy(xy: impl Into<XY>) -> Self {
    //     let XY(x, y) = xy.into();
    //     Self { lat: y, lon: x }
    // }

    /// `tky2jgd.par` を用いて [`JGD2000`] へ変換する
    pub fn to_jgd2000(self) -> Jgd2000 {
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

/// Tokyo97 座標系。
/// Tokyo97, the older Japanese Datum.
///
/// 3パラメータを用いた変換式[^1]で定義される旧日本測地系。
/// [^1]: [飛田幹男(1997) 最近の測地座標系と座標変換についての考察](https://www.jstage.jst.go.jp/article/sokuchi1954/43/4/43_4_231/_pdf)
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

/// 世界測地系 JGD2000.
///
/// Japanese Geodetic Datum 2000 (JGD2000).
///
/// EPSG:4612
pub struct Jgd2000 {
    degeee: Degree,
}

impl Jgd2000 {
    // GRS80楕円体

    /// `touhokutaiheiyouoki2011.par` を用いて [`JGD2011`] へ変換する
    pub fn to_jgd2011(self) -> Jgd2011 {
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

/// 世界測地系 JGD2011.
pub struct Jgd2011 {
    degree: Degree,
}

impl Jgd2011 {
    pub fn xy(self) -> [f64; 2] {
        self.degree.xy()
    }

    // pub fn latlon(self) -> LatLon {
    //     todo!()
    // }

    /// `touhokutaiheiyouoki2011.par` を用いて [`JGD2000`] へ逆変換する
    pub fn to_jgd2000(self) {}
}

// /// 平面直角座標系
// https://vldb.gsi.go.jp/sokuchi/surveycalc/surveycalc/algorithm/xy2bl/xy2bl.htm
// https://sw1227.hatenablog.com/entry/2018/11/30/200702
// pub struct PlaneRectangular<T>(T);

// /// Webメルカトル座標系
// pub struct WebMercator<T>(T);
