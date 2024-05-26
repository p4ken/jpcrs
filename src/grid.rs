//! Parameters grid.

use std::{mem, slice};

use crate::LatLon;

#[repr(align(4))] // <- this is why the bytes wrapped with the struct.
pub struct Bin<const N: usize>(pub [u8; N]);
impl<const N: usize> Bin<N> {
    pub const fn to_grid(&self) -> Grid {
        assert!(isize::MAX as usize > N);
        let data = self.0.as_ptr() as *const Dot;
        let len = self.0.len() / mem::size_of::<Dot>();

        #[cfg(not(target_endian = "little"))]
        compile_error!("compile target must be little endian");
        // SAFETY:
        // `data` is aligned as same as `Dot`. It outlives for the lifetime of `self`.
        // `len * size_of::<Recod>()` always within length of `data` and smaller than `isize::MAX`.
        // Returned value used as immutable.
        let dots = unsafe { slice::from_raw_parts(data, len) };

        Grid::new(dots)
    }
}

pub struct Grid<'a> {
    pub(crate) dots: &'a [Dot],
}
impl<'a> Grid<'a> {
    const fn new(dots: &'a [Dot]) -> Self {
        Self { dots }
    }
    pub fn interpolate(&self, bl: LatLon) -> LatLon {
        todo!()
    }
    pub fn interpolate_wide(&self, bl: LatLon) -> LatLon {
        todo!()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Dot {
    pub(crate) grid_lat: i16,
    pub(crate) grid_lon: i16,
    pub(crate) d_lat_us: i32,
    pub(crate) d_lon_us: i32,
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
    fn grid_interpolate() {
        let dots = [];
        let sut = Grid::new(&dots);
    }
}
