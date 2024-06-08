mod coord;
mod crs;
mod grid;
mod island;
mod neo;
mod neo2;
mod par;

pub use coord::Degree_;
pub use crs::{Jgd2000, Jgd2011, Tokyo, Tokyo97};
pub use grid::Grid;
#[cfg(feature = "tky2jgd")]
pub use grid::TKY2JGD;
use neo::Degree;
use neo2::Geodetic;
pub use neo2::{LatLon, LonLat};

///
/// 引数は緯度、経度の順です。ご注意ください。
/// x と y で表される座標をお持ちの場合、y, x の順で渡す必要があります。
///
/// `f64` に `Into<Degree>` が実装されているため、度単位の `f64` をそのまま引数に渡すことができます。
pub fn from_tokyo<T: Into<Degree>>(lat: T, lon: T) -> Tokyo<T> {
    todo!()
}

pub fn from_tokyo_<T: Geodetic<Degree>>(degree: T) -> Tokyo<T> {
    todo!()
}

// pub fn xy(xy: [f64; 2]) -> Degree {
//     Degree::with_xy(xy)
// }
