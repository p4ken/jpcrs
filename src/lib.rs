//! 日本の座標の測地系を変換する。
//! Transform geodetic coordinate systems used in Japan.
//!
//! # Examples
//!
//! 旧日本測地系 [`Tokyo`] の北緯35度・東経135度を、世界測地系 [`Jgd2011`] に変換する。
//!
//! ```no_run
//! # #[cfg(all(feature = "tky2jgd", feature = "patchjgd"))]
//! let (lat, lon) = jgd::from_tokyo(35.0, 135.0)
//!     .to_jgd2000()
//!     .to_jgd2011()
//!     .lat_lon();
//! ```
//!
//! <br>
//!
//! [`geo::geometry`] の測地系を変換する。
//!
//! ```no_run
//! use geo::{Coord, LineString, MapCoords};
//!
//! let tokyo_datum = LineString::from(vec![(135.0, 35.0), (135.1, 35.1)]);
//! let jgd2011 = tokyo_datum.map_coords(|Coord { x, y }| {
//!     // 順序に注意: lat, lon <-> y, x
//! #   #[cfg(all(feature = "tky2jgd", feature = "patchjgd"))]
//!     let (y, x) = jgd::from_tokyo(y, x).to_jgd2000().to_jgd2011().lat_lon();
//!     Coord { x, y }
//! });
//! ```
//!
//! # Limitations
//!
//! 日本国内の陸地の座標を対象としている。海上や国外には適さない。
//!
//! 測地系変換によって、ある測地系で測量、作成された座標を、あたかも別の測地系かのように「再現」できる。
//! 異なる測地系で整備された座標同士のズレを「低減」できても、ズレが消滅することはない。
//! 変換メソッド毎に精度や制約が異なり、詳細はそれぞれのドキュメントに記載されている。
//!
//! 度単位の緯度経度のみ変換可能。平面直角座標系は未対応。
//!
//! # Features
//!
//! 有効にすると、ビルド後のバイナリサイズが増える。
//!
//! - `tky2jgd` - [TKY2JGD] を使用する。デフォルトで有効。
//! - `patchjgd` - [TOUHOKUTAIHEIYOUOKI2011] を使用する。デフォルトで有効。
//!
//! # References
//!
//! - 飛田幹男 [最近の測地座標系と座標変換についての考察](https://www.jstage.jst.go.jp/article/sokuchi1954/43/4/43_4_231/_pdf) (測地学会誌 43巻 4号 (1997) pp231-235)
//! - 飛田幹男 [世界測地系移行のための座標変換ソフトウェア "TKY2JGD"](https://www.gsi.go.jp/common/000063173.pdf) (国土地理院時報 97集 (2002) pp31-51)
//! - 飛田幹男ほか [日本測地系における離島位置の補正量](https://www.jstage.jst.go.jp/article/sokuchi1954/49/3/49_3_181/_pdf) (測地学会誌 49巻 3号 (2003) pp181-192)
//! - 飛田幹男 [地震時地殻変動に伴う座標値の変化を補正するソフトウェア "PatchJGD"](https://www.jstage.jst.go.jp/article/sokuchi/55/4/55_4_355/_pdf/-char/ja) (測地学会誌 55巻 4号 (2009) pp355-367)

mod coord;
mod crs;
mod grid;
mod island;
mod par;

use coord::{LatLon, LonLat};
pub use crs::{
    from_jgd2000, from_jgd2011, from_tokyo, from_tokyo97, Jgd2000, Jgd2011, Tokyo, Tokyo97,
};
use grid::Grid;
#[cfg(feature = "tky2jgd")]
pub use grid::TKY2JGD;
#[cfg(feature = "patchjgd")]
pub use grid::TOUHOKUTAIHEIYOUOKI2011;
