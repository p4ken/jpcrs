//! Parameters grid.

use std::{mem, slice};

const TKY2JGD: Table = Table::from_bytes(include_bytes!("../par/TKY2JGD.in"));

struct Table {
    bytes: &'static [u8],
}
impl Table {
    const fn from_bytes(bytes: &'static [u8]) -> Self {
        Self { bytes }
    }
    const fn records(&self) -> &[Record] {
        let data = self.bytes.as_ptr() as _;
        let len = self.bytes.len() / mem::size_of::<Record>();

        #[cfg(not(target_endian = "little"))]
        compile_error!("target must be little endian");
        unsafe { slice::from_raw_parts(data, len) }
    }
}

#[derive(Debug)]
#[repr(C)]
struct Record {
    grid_lat: i16,
    grid_lon: i16,
    d_lat_us: i32,
    d_lon_us: i32,
}
impl Record {
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
    fn table_records() {
        let records = TKY2JGD.records();
        assert_eq!(records.len(), 392323);

        let r = records.last().unwrap();
        assert_eq!(r.grid_lat, 5463);
        assert_eq!(r.grid_lon, 3356);
        assert_eq!(r.d_lat_us, 7875320);
        assert_eq!(r.d_lon_us, -13995610);
    }
}
