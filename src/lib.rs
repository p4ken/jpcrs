mod coord;
mod crs;
mod grid;
mod island;
mod par;

pub use coord::Degree;
pub use crs::{Jgd2000, Jgd2011, Tokyo, Tokyo97};
pub use grid::Grid;
#[cfg(feature = "tky2jgd")]
pub use grid::TKY2JGD;

pub fn from_tokyo(xy: [Degree; 2]) -> Tokyo {
    todo!()
}
