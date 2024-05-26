mod coord;
mod crs;
mod grid;
mod island;
mod par;

pub use coord::{Degree, LatLon, XY};
pub use crs::{Tokyo97, TokyoDatum, JGD2000, JGD2011};
pub use grid::Grid;
#[cfg(feature = "tky2jgd")]
pub use grid::TKY2JGD;

pub fn from_tokyo(p: impl Into<XY>) -> TokyoDatum {
    todo!()
}
