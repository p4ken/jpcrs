mod bl;
mod island;
mod par;
mod patchjgd;
mod tky2jgd;

/// 日本測地系 Tokyo Datum
pub struct TokyoDatum<T>(T);

/// 日本測地系 Tokyo97
pub struct Tokyo97<T>(T);

/// 世界測地系 JGD2000
pub struct JGD2000<T>(T);

/// 世界測地系 JGD2011
pub struct JGD2011<T>(T);

/// 平面直角座標系
// https://vldb.gsi.go.jp/sokuchi/surveycalc/surveycalc/algorithm/xy2bl/xy2bl.htm
// https://sw1227.hatenablog.com/entry/2018/11/30/200702
// pub struct PlaneRectangular<T>(T);

/// Webメルカトル座標系
// pub struct WebMercator<T>(T);

impl<T> TokyoDatum<T> {
    // Bessel楕円体

    /// `tky2jgd.par` を用いて [`JGD2000`] へ変換する
    pub fn to_jgd2000() {
        // 国土地理院時報(2002，97集)「世界測地系移行のための座標変換ソフトウェア”TKY2JGD"」
        // https://www.gsi.go.jp/common/000063173.pdf
        // > 地域毎の変換パラメータの格子点は，3 次メッシュの中央ではなく，南西隅に対応する。
    }

    /// 離島位置の補正量を用いて [`Tokyo97`] へ変換する
    pub fn to_tokyo97() {
        // 日本測地系における離島位置の補正量
        // https://www.jstage.jst.go.jp/article/sokuchi1954/49/3/49_3_181/_pdf
    }
}

impl<T> Tokyo97<T> {
    /// 3パラメータを用いて [`JGD2000`] へ変換する
    pub fn to_jgd2000() {}

    /// 離島位置の補正量を用いて [`TokyoDatum`] へ逆変換する
    pub fn to_tokyo() {
        // 日本測地系における離島位置の補正量
        // https://www.jstage.jst.go.jp/article/sokuchi1954/49/3/49_3_181/_pdf
    }
}

impl<T> JGD2000<T> {
    // GRS80楕円体

    /// `touhokutaiheiyouoki2011.par` を用いて [`JGD2011`] へ変換する
    pub fn to_jgd2011() {
        // 地震時地殻変動に伴う座標値の変化を補正するソフトウェア“PatchJGD”
        // https://www.jstage.jst.go.jp/article/sokuchi/55/4/55_4_355/_pdf/-char/ja
    }

    /// `tky2jgd.par` を用いて [`TokyoDatum`] へ逆変換する
    pub fn to_tokyo() {}

    /// 3パラメータを用いて [`Tokyo97`] へ逆変換する
    pub fn to_tokyo97() {
        // https://www.drm.jp/jisx0410/JisGridSystem_1_Geoid.html

        // https://www.jstage.jst.go.jp/article/sokuchi1954/43/4/43_4_231/_pdf
        // > このパラメータによって定義される日本測地系がTokyo97である
    }
}

impl<T> JGD2011<T> {
    /// `touhokutaiheiyouoki2011.par` を用いて [`JGD2000`] へ逆変換する
    pub fn to_jgd2000() {}
}
