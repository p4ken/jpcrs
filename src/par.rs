//! Parameters grid

// rlib: +19MB
// pub const TKY2JGD_FLAT: &[u8] = include_bytes!("../par/TKY2JGD.in");

// rlib: +4.5MB
const TKY2JGD: Table = Table(include_bytes!("../par/TKY2JGD.in"));

struct Table(pub &'static [u8]);
impl Table {}

#[derive(Debug)]
struct Record {
    grid_lat: i16,
    grid_lon: i16,
    d_lat_us: i32,
    d_lon_us: i32,
}
impl Record {
    fn from_binary(bin: &[u8]) -> Self {
        Self {
            grid_lat: i16::from_le_bytes(bin[0..2].try_into().unwrap()),
            grid_lon: i16::from_le_bytes(bin[2..4].try_into().unwrap()),
            d_lat_us: i32::from_le_bytes(bin[4..8].try_into().unwrap()),
            d_lon_us: i32::from_le_bytes(bin[8..12].try_into().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tky2jgd_last() {
        let r = Record::from_binary(&TKY2JGD.0[12 * 392322..]);
        assert_eq!(r.grid_lat, 5463);
        assert_eq!(r.grid_lon, 3356);
        assert_eq!(r.d_lat_us, 7875320);
        assert_eq!(r.d_lon_us, -13995610);
    }
}
