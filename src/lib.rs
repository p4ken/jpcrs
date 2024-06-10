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

mod coord;
mod crs;
mod grid;
mod island;
mod par;

use coord::{LatLon, LonLat};
pub use crs::{from_tokyo, Jgd2000, Jgd2011, Tokyo, Tokyo97};
use grid::Grid;
#[cfg(feature = "tky2jgd")]
pub use grid::TKY2JGD;
#[cfg(feature = "patchjgd")]
pub use grid::TOUHOKUTAIHEIYOUOKI2011;
