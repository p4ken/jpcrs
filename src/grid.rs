//! Parameters grid.

use crate::{par, LatLon, XY};

#[cfg(feature = "tky2jgd")]
pub const TKY2JGD: Grid = par::TKY2JGD.to_grid();

#[derive(Debug)]
pub struct Grid<'a> {
    dots: &'a [Dot],
}
impl<'a> Grid<'a> {
    pub const fn new(dots: &'a [Dot]) -> Self {
        Self { dots }
    }
    pub fn interpolate(&self, xy: impl Into<XY>) -> XY {
        todo!()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Dot {
    index: Mesh3Index,
    shift: MicroSecond,
}
impl Dot {
    // fn to_key(&self) -> MilliSecond {
    //     MilliSecond(Geodetic {
    //         lat: i32::from(self.grid_lat) * 30 * 1000,
    //         lon: i32::from(self.grid_lon) * 45 * 1000,
    //     })
    // }
}

#[derive(Debug)]
#[repr(C)]
struct Mesh3Index {
    lat: i16,
    lon: i16,
}

#[derive(Debug)]
#[repr(C)]
pub struct MicroSecond {
    lat: i32,
    lon: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "tky2jgd")]
    #[test]
    fn tky2jgd_grid() {
        let records = TKY2JGD.dots;
        assert_eq!(records.len(), 392323);

        let r = records.last().unwrap();
        assert_eq!(r.index.lat, 5463);
        assert_eq!(r.index.lon, 3356);
        assert_eq!(r.shift.lat, 7875320);
        assert_eq!(r.shift.lon, -13995610);
    }

    #[test]
    fn grid_interpolate() {
        let dots = [];
        let sut = Grid::new(&dots);
    }
}
