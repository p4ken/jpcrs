use crate::grid::{Bin, Grid};

const TKY2JGD_BIN: Bin<4707876> = Bin(*include_bytes!("../par/TKY2JGD.in"));

const TKY2JGD: Grid = TKY2JGD_BIN.to_grid();

pub fn tky2jgd(lat: i32, lon: i32) -> (i32, i32) {
    (0, 0)
}

pub fn inverse() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tky2jgd_grid() {
        let records = TKY2JGD.dots;
        assert_eq!(records.len(), 392323);

        let r = records.last().unwrap();
        assert_eq!(r.grid_lat, 5463);
        assert_eq!(r.grid_lon, 3356);
        assert_eq!(r.d_lat_us, 7875320);
        assert_eq!(r.d_lon_us, -13995610);
    }
}
