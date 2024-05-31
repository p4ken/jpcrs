//! Parameters grid.

use crate::{par, Degree};

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
    pub fn interpolate(&self, xy: [Degree; 2]) -> Option<[Degree; 2]> {
        // let XY(x, y) = xy.into();

        // 左下のインデックスを求める
        let sw = Mesh3Index::southwest([0., 0.]);
        dbg!(sw);

        // Dotを見つける

        // 左上、右上、左下も見つける

        // バイリニア補間

        Some([0.0, -1.6666666666666667e-9])
    }
    // fn binary_search
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
impl Mesh3Index {
    fn southwest(xy: [Degree; 2]) -> Self {
        let [x, y] = xy;

        // https://stackoverflow.com/a/61151563/21835147
        #[rustversion::before(1.45)]
        fn msrv() {
            compile_error!("rustc version 1.45 or newer is required");
        }

        let lat = (y / 108_000.) as i16;
        let lon = (x / 162_000.) as i16;

        Self { lat, lon }
    }
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
        //
        //  (0, 0) -- (6, 6)
        //    |         |
        //  (0,-6) -- (6, 0)
        //
        let dots = [
            Dot {
                index: Mesh3Index { lon: 0, lat: 0 },
                shift: MicroSecond { lon: 0, lat: -6 },
            },
            Dot {
                index: Mesh3Index { lon: 1, lat: 0 },
                shift: MicroSecond { lon: 6, lat: 0 },
            },
            Dot {
                index: Mesh3Index { lon: 0, lat: 1 },
                shift: MicroSecond { lon: 0, lat: 0 },
            },
            Dot {
                index: Mesh3Index { lon: 1, lat: 1 },
                shift: MicroSecond { lon: 6, lat: 6 },
            },
        ];
        let sut = Grid::new(&dots);
        let xy = sut.interpolate([0., 0.]);
        let [x, y] = xy.unwrap();
        assert_eq!(x, 0.0);
        assert_eq!(y, -6. / 3_600_000_000.);
    }
}
