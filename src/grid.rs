//! Parameters grid.

use std::{mem, ops::Deref, slice};

use crate::LatLon;

// const TKY2JGD: Par_ = Par_::from_bytes(include_bytes!("../par/TKY2JGD.in"));

const TKY2JGD_BIN: Bin<4707876> = Bin(*include_bytes!("../par/TKY2JGD.in"));

const TKY2JGD: Grid = TKY2JGD_BIN.to_grid();

// const TKY2JGD_G: GridPar<4707876> = GridPar::new(Bin(*include_bytes!("../par/TKY2JGD.in")));

#[repr(align(4))]
struct Bin<const N: usize>([u8; N]);
impl<const N: usize> Bin<N> {
    const fn to_grid(&self) -> Grid {
        let data = self.0.as_ptr() as *const Dot;
        let len = self.0.len() / mem::size_of::<Dot>();
        let dots = unsafe { slice::from_raw_parts(data, len) };
        Grid::new(dots)
    }
}

// struct GridPar<'a, const N: usize> {
//     bin: Bin<N>,
//     grid: Grid<'a>,
// }
// impl<const N: usize> GridPar<'_, N> {
//     const fn new(bin: Bin<N>) -> Self {
//         let grid = bin.to_grid();
//         Self { bin, grid }
//     }
// }

const TKY2JGD_S: Grid = TKY2JGD_BIN.to_grid();

// pub fn interpolate(&self, bl: LatLon, par: &Par)

// struct Par_ {
//     bytes: &'static [u8],
// }
// impl Par_ {
//     const fn from_bytes(bytes: &'static [u8]) -> Self {
//         Self { bytes }
//     }
//     fn dots(&self) -> &[Dot] {
//         let data = self.bytes.as_ptr() as _;
//         let len = self.bytes.len() / mem::size_of::<Dot>();

//         #[cfg(not(target_endian = "little"))]
//         compile_error!("compile target must be little endian");
//         // SAFETY:
//         // - `len * size_of::<Recod>()` always within length of `data`.
//         // - `data` has 'static lifetime.
//         // - `len * size_of::<Recod>()` is smaller than `isize::MAX` statically.
//         unsafe { slice::from_raw_parts(data, len) }
//     }
// }
// impl From<Par> for &[Dot] {
//     fn from(value: Par) -> Self {
//         todo!()
//     }
// }

// trait Grid_ {}
// impl<'a, T: Into<&'a [Dot]>> Grid_ for T {}

struct Grid<'a> {
    dots: &'a [Dot],
}
impl<'a> Grid<'a> {
    const fn new(dots: &'a [Dot]) -> Self {
        Self { dots }
    }
}

struct Grid_<const N: usize> {
    dots: [Dot; N],
}
impl<const N: usize> Grid_<N> {
    const fn new(dots: [Dot; N]) -> Self {
        Self { dots }
    }
    // const fn from_bytes<const M: usize>(&self, bytes: [u8; M]) -> Grid_<N> {
    //     let dots: [Dot; N] = unsafe { mem::transmute(bytes) };
    //     Grid { dots }
    // }
    pub fn interpolate(&self, bl: LatLon) -> LatLon {
        todo!()
    }
    pub fn interpolate_wide(&self, bl: LatLon) -> LatLon {
        todo!()
    }
}

#[derive(Debug)]
#[repr(C)]
struct Dot {
    grid_lat: i16,
    grid_lon: i16,
    d_lat_us: i32,
    d_lon_us: i32,
}
impl Dot {
    // fn to_key(&self) -> MilliSecond {
    //     MilliSecond(Geodetic {
    //         lat: i32::from(self.grid_lat) * 30 * 1000,
    //         lon: i32::from(self.grid_lon) * 45 * 1000,
    //     })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_records() {
        let records = TKY2JGD.dots;
        assert_eq!(records.len(), 392323);

        let r = records.last().unwrap();
        assert_eq!(r.grid_lat, 5463);
        assert_eq!(r.grid_lon, 3356);
        assert_eq!(r.d_lat_us, 7875320);
        assert_eq!(r.d_lon_us, -13995610);
    }

    #[test]
    fn grid_interpolate() {
        let dots = [];
        let sut = Grid::new(&dots);
    }
}
