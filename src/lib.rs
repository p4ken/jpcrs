//! 日本の測地座標を変換する。
//! Transform geodetic coordinates used in Japan.
//!
//! 日本国内の地表面の座標のみを対象としている。海上、空中、地中、国外には適さない。
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
//! 飛田幹男, 1997: 最近の測地座標系と座標変換についての考察 <https://www.jstage.jst.go.jp/article/sokuchi1954/43/4/43_4_231/_pdf>
//!
//! 国土地理院, 2003: TKY2JGD.par Ver.2.1.2 <https://www.gsi.go.jp/sokuchikijun/tky2jgd_download.html>
//!
//! 国土地理院, 2017: touhokutaiheiyouoki2011.par Ver.4.0.0 <https://www.gsi.go.jp/sokuchikijun/sokuchikijun41012.html>

mod coord;
mod crs;
mod grid;
mod island;
mod par;

use coord::{LatLon, LonLat};
pub use crs::{from_tokyo, Jgd2000, Jgd2011, Tokyo, Tokyo97};
pub use grid::Grid;
#[cfg(feature = "tky2jgd")]
pub use grid::TKY2JGD;
#[cfg(feature = "patchjgd")]
pub use grid::TOUHOKUTAIHEIYOUOKI2011;
