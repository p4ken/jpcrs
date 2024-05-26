use std::{mem, slice};

use crate::{grid::Dot, Grid};

#[cfg(feature = "tky2jgd")]
pub const TKY2JGD: Bin<4707876> = Bin(*include_bytes!("../par/TKY2JGD.in"));

pub fn tky2jgd(lat: i32, lon: i32) -> (i32, i32) {
    (0, 0)
}

pub fn inverse() {}

#[derive(Debug)]
#[repr(align(4))] // <- this is why the bytes are wrapped.
pub struct Bin<const N: usize>([u8; N]);
impl<const N: usize> Bin<N> {
    pub const fn to_grid(&self) -> Grid {
        assert!(isize::MAX as usize > N);
        let data = self.0.as_ptr() as *const Dot;
        let len = self.0.len() / mem::size_of::<Dot>();

        #[cfg(not(target_endian = "little"))]
        compile_error!("compile target must be little endian");
        // SAFETY:
        // `data` is single allocated and aligned as same as `Dot`.
        // `len * element size` is within the length of `data` and is smaller than `isize::MAX`.
        // Returned value is immutable. It's lifetime is same as `data`.
        let dots = unsafe { slice::from_raw_parts(data, len) };

        Grid::new(dots)
    }
}
