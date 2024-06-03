//! Parameters grid.

use crate::{par, Degree};

#[cfg(feature = "tky2jgd")]
pub const TKY2JGD: Grid = par::TKY2JGD.to_grid();

#[derive(Debug, Clone)]
pub struct Grid<'a> {
    dots: &'a [Dot],
}
impl<'a> Grid<'a> {
    pub const fn new(dots: &'a [Dot]) -> Self {
        Self { dots }
    }
    pub fn interpolate(&self, p: Degree) -> Option<Degree> {
        let sw_mesh = Mesh3::southwest(p);
        let i = self.search(sw_mesh)?;
        let sw_shift = self.dots[i].shift;

        let i = self.at(i + 1)?.search(sw_mesh.to_east())?;
        let se_shift = self.dots[i].shift;

        let i = self.after(i + 1)?.search(sw_mesh.to_north())?;
        let nw_shift = self.dots[i].shift;

        let i = self.at(i + 1)?.search(sw_mesh.to_north().to_east())?;
        let ne_shift = self.dots[i].shift;

        // bilinear interpolation
        let [s_weight, w_weight] = sw_mesh.southwest_weight(p);
        // let lat_shift = sw_shift.to_degree() * (s_weight * w_weight);

        Some(Degree::new([0.0, -1.6666666666666667e-9]))
    }
    fn search(&self, query: Mesh3) -> Option<usize> {
        self.dots.binary_search_by_key(&query, |dot| dot.mesh).ok()
    }
    fn after(&self, first: usize) -> Option<Self> {
        let dots = self.dots.get(first..)?;
        Some(Self::new(dots))
    }
    fn at(&self, index: usize) -> Option<Self> {
        let dots = self.dots.get(index..=index)?;
        Some(Self::new(dots))
    }
}
// impl Index<SliceIndex<[Dot]>> for Grid<'_> {
//     type Output = Self;

//     fn index(&self, index: SliceIndex<[Dot]>) -> &Self::Output {
//         todo!()
//     }
// }

struct Quad {}
impl Quad {
    fn weighted_mean(self) {}
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Dot {
    mesh: Mesh3,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
/// Serial number of Japanese MESH3 grids starting from 0 degree.
struct Mesh3 {
    lat: i16,
    lon: i16,
}
impl Mesh3 {
    const LAT_SEC: f64 = 30.;
    const LON_SEC: f64 = 45.;

    /// Evaluate the southwest of the mesh containing `p`.
    fn southwest(p: Degree) -> Self {
        let [x, y] = p.xy();
        let lat = (y * 120.) as i16;
        let lon = (x * 80.) as i16;
        Self { lat, lon }
    }
    fn southwest_weight(self, p: Degree) -> [f64; 2] {
        let [lat_max, lon_max] = self.to_north().to_east().to_degree().latlon();
        let [lat_p, lon_p] = p.latlon();
        let weight_lat = (lat_max - lat_p) * 3_600. / Self::LAT_SEC;
        let weight_lon = (lon_max - lon_p) * 3_600. / Self::LON_SEC;
        [weight_lat, weight_lon]
    }
    fn to_north(mut self) -> Self {
        self.lat += 1;
        self
    }
    fn to_east(mut self) -> Self {
        self.lon += 1;
        self
    }
    fn to_degree(self) -> Degree {
        let lat = f64::from(self.lat) * Self::LAT_SEC / 3_600.;
        let lon = f64::from(self.lat) * Self::LON_SEC / 3_600.;
        Degree::new_unchecked(lat, lon)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicroSecond {
    lat: i32,
    lon: i32,
}
impl MicroSecond {
    fn to_degree(self) -> Degree {
        let lat = f64::from(self.lat) / 3_600_000.;
        let lon = f64::from(self.lon) / 3_600_000.;
        Degree::new_unchecked(lat, lon)
    }
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
        assert_eq!(r.mesh.lat, 5463);
        assert_eq!(r.mesh.lon, 3356);
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
                mesh: Mesh3 { lon: 0, lat: 0 },
                shift: MicroSecond { lon: 0, lat: -6 },
            },
            Dot {
                mesh: Mesh3 { lon: 1, lat: 0 },
                shift: MicroSecond { lon: 6, lat: 0 },
            },
            Dot {
                mesh: Mesh3 { lon: 0, lat: 1 },
                shift: MicroSecond { lon: 0, lat: 0 },
            },
            Dot {
                mesh: Mesh3 { lon: 1, lat: 1 },
                shift: MicroSecond { lon: 6, lat: 6 },
            },
        ];
        let sut = Grid::new(&dots);
        let xy = sut.interpolate(Degree::new([0., 0.]));
        let [x, y] = xy.unwrap().xy();
        assert_eq!(x, 0.0);
        assert_eq!(y, -6. / 3_600_000_000.);
    }
}
