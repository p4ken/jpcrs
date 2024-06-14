use crate::LatLon;

#[cfg(feature = "tky2jgd")]
use crate::TKY2JGD;

#[cfg(feature = "patchjgd")]
use crate::TOUHOKUTAIHEIYOUOKI2011;

/// 旧日本測地系からの変換を開始する。
/// Transform from Tokyo Datum.
///
/// 世界測地系で測量された座標の場合は、[`Tokyo97`] ...
///
/// # Examples
///
/// ```
/// jgd::from_tokyo(35.0, 135.0);
/// ```
pub fn from_tokyo(lat: f64, lon: f64) -> Tokyo {
    Tokyo::new(LatLon(lat, lon))
}

pub fn from_tokyo97(lat: f64, lon: f64) -> Tokyo97 {
    // Tokyo97::new(LatLon(lat, lon))
    todo!()
}

pub fn from_jgd2000(lat: f64, lon: f64) -> Jgd2000 {
    Jgd2000::new(LatLon(lat, lon))
}

pub fn from_jgd2011(lat: f64, lon: f64) -> Jgd2000 {
    Jgd2000::new(LatLon(lat, lon))
}

// pub fn from_tokyo_(p: impl Into<LatLon>) -> Tokyo {
//     Tokyo(p.into())
// }

/// 旧日本測地系。Tokyo Datum, The older Japanese Datum.
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

    /// [`TKY2JGD`] を用いて [`Jgd2000`] へ変換する。
    /// Transform to JGD2000.
    ///
    /// # Limitations
    ///
    /// 日本国内の地表面の座標のみに使用できる。地中や空中ではズレが大きくなる。
    ///
    /// パラメータグリッドの範囲外の座標は、[`Tokyo97`] を経由し、一律の数式による変換にフォールバックされる。
    /// たとえ陸地であっても、無人島や、後年に埋め立てられた沿岸部などで、パラメータグリッドが欠損している。
    /// 複数の座標で表される形状が、パラメータグリッドの範囲内外をまたがっていると、変換後の形状が大きく変わる可能性がある。
    ///
    /// 国土地理院によるオリジナルの実装の精度は、一定条件下で「緯度, 経度の標準偏差はそれぞれ9cm, 8cm」[(飛田, 2002)](crate#references) とされている。
    #[cfg(feature = "tky2jgd")]
    pub fn to_jgd2000(&self) -> Jgd2000 {
        match TKY2JGD.interpolate(self.lat_lon) {
            Some(shift) => Jgd2000::new(self.lat_lon + shift),
            None => self.to_tokyo97().to_jgd2000(),
        }
    }

    /// 離島位置の補正量 [(飛田, 2003)](crate#references) を用いて [`Tokyo97`] へ変換する。
    pub fn to_tokyo97(&self) -> Tokyo97 {
        // https://www.drm.jp/jisx0410/JisGridSystem_1_Geoid.html
        todo!()
    }
}

/// 旧日本測地系。Tokyo 97, The older Japanese Datum.
///
/// 世界測地系を基準に、3パラメータによる変換式で定義された測地系 [(飛田, 1997)](crate#references)。
pub struct Tokyo97 {
    lat_lon: LatLon,
}
impl Tokyo97 {
    /// 3パラメータによる変換式 [(飛田, 1997)](crate#references) を用いて [`Jgd2000`] へ変換する。
    /// Transform to JGD2000.
    pub fn to_jgd2000(&self) -> Jgd2000 {
        todo!()
    }

    /// 離島位置の補正量 [(飛田, 2003)](crate#references) を用いて [`Tokyo`] へ逆変換する。
    /// Inverse of [`Tokyo::to_tokyo97`].
    pub fn to_tokyo(&self) {}
}

/// 世界測地系。Japanese Geodetic Datum 2000 (JGD2000).
///
/// EPSG: 4612
pub struct Jgd2000 {
    lat_lon: LatLon,
}

impl Jgd2000 {
    // GRS80楕円体

    fn new(lat_lon: impl Into<LatLon>) -> Self {
        let lat_lon = lat_lon.into();
        Self { lat_lon }
    }

    /// [`TOUHOKUTAIHEIYOUOKI2011`] を用いて [`Jgd2011`] へ変換する。
    ///
    /// パラメータグリッド内の一部地域に欠損があり、...
    ///
    /// 現在のところ、東日本大震災による地殻変動分しか補正されない。
    /// ただし、今後のバージョンアップで他のいくつかの地殻変動分を追加する可能性がある。
    /// いずれにしても ...
    pub fn to_jgd2011(&self) -> Jgd2011 {
        // todo
        Jgd2011::new(self.lat_lon)
    }

    /// [`TKY2JGD`] を用いて [`Tokyo`] へ逆変換する。
    /// Inverse of [`Tokyo::to_jgd2000`].
    ///
    /// 今後、あらゆるデータが世界測地系で測量、作成されるため、なるべく本メソッドは使わない方が良い。
    /// 新旧の測地系の座標を重ねる用途であれば、原則として旧日本測地系から世界測地系へ変換した方が良い。
    pub fn to_tokyo(&self) {}

    /// 3パラメータを用いて [`Tokyo97`] へ逆変換する。
    /// Inverse of [`Tokyo97::to_jgd2000`].
    pub fn to_tokyo97(&self) {}
}

/// 世界測地系。Japanese Geodetic Datum 2011 (JGD2011).
///
/// EPSG: 6668
pub struct Jgd2011 {
    lat_lon: LatLon,
}
impl Jgd2011 {
    fn new(lat_lon: impl Into<LatLon>) -> Self {
        let lat_lon = lat_lon.into();
        Self { lat_lon }
    }

    /// 度単位の緯度経度。
    /// Latitude and longitude in degrees.
    pub fn lat_lon(&self) -> (f64, f64) {
        self.lat_lon.into()
    }

    // pub fn lon_lat(&self) -> (f64, f64) {
    //     self.lat_lon.rev()
    // }

    /// [`TOUHOKUTAIHEIYOUOKI2011`] を用いて [`Jgd2000`] へ逆変換する。
    pub fn to_jgd2000(&self) {}
}

// /// 平面直角座標系
// https://vldb.gsi.go.jp/sokuchi/surveycalc/surveycalc/algorithm/xy2bl/xy2bl.htm
// https://sw1227.hatenablog.com/entry/2018/11/30/200702
// pub struct PlaneRectangular<T>(T);

// /// Webメルカトル座標系
// pub struct WebMercator<T>(T);
