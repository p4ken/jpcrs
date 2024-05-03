pub struct JGD2000<T>(T);
pub struct JGD2011<T>(T);
// pub struct ITRF94<T>(T);
// pub struct WGS84<T>(T);

/// 旧日本測地系
pub struct TokyoDatum<T>(T);

/// ITRF94 座標系から3パラメータを用いて変換された、旧日本測地系
///
/// https://www.jstage.jst.go.jp/article/sokuchi1954/43/4/43_4_231/_pdf
/// > ITRF94からTokyo97系への変換パラメータは,
/// > T1=146.414m,T2=-507.337m,
/// > T3=-680.50m,D=0.00,R1=0.00,
/// > R2=0.00,R3=0.00
/// > である.これが,日本の測地関係者が長年待ち望んでいた座標変換パラメータであり,このパラメータによって定義される日本測地系がTokyo97である.
// pub struct Tokyo97<T>(T);

/// 3パラメータによる変換
pub fn shift_islands() {
    // 日本測地系における離島位置の補正量
    // https://www.jstage.jst.go.jp/article/sokuchi1954/49/3/49_3_181/_pdf
}
pub fn rev_shift_islands() {}

/// パラメータファイル `tky2jgd.par` を用いて `JGD2000` に変換する。
pub fn tky2jgd_par() {
    // 国土地理院時報(2002，97集)「世界測地系移行のための座標変換ソフトウェア”TKY2JGD"」
    // https://www.gsi.go.jp/common/000063173.pdf
    // > 地域毎の変換パラメータの格子点は，3 次メッシュの中央ではなく，南西隅に対応する。
}

/// 3パラメータを用いて幾何学的に世界測地系に変換する
pub fn tky2jgd_3params() {
    // https://www.drm.jp/jisx0410/JisGridSystem_1_Geoid.html
}

/// パラメータファイル `touhokutaiheiyouoki2011.par` を用いて `JGD2011` に変換する。
pub fn touhokutaiheiyouoki2011_par() {}
