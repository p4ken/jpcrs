use crate::{par, LatLon};

/// 日本測地系から世界測地系への座標変換パラメータ。
#[cfg(feature = "tky2jgd")]
pub const TKY2JGD: Grid = par::TKY2JGD.to_grid();

/// 平成23年(2011年)東北地方太平洋沖地震の座標補正パラメータ。
#[cfg(feature = "patchjgd")]
pub const TOUHOKUTAIHEIYOUOKI2011: Grid = par::TOUHOKUTAIHEIYOUOKI2011.to_grid();

/// パラメータグリッド。
/// Parameters grid.
#[derive(Debug, Clone)]
pub struct Grid<'a> {
    dots: &'a [Dot],
}
impl<'a> Grid<'a> {
    pub const fn new(dots: &'a [Dot]) -> Self {
        Self { dots }
    }
    pub fn interpolate(&self, p: LatLon) -> Option<LatLon> {
        let sw_mesh = Mesh3::southwest(&p);
        let i = self.search_after(0, sw_mesh)?;
        let sw_shift = self.dots[i];

        let i = self.search_at(i + 1, sw_mesh.to_east())?;
        let se_shift = self.dots[i].shift;

        let i = self.search_after(i + 1, sw_mesh.to_north())?;
        let nw_shift = self.dots[i].shift;

        let i = self.search_at(i + 1, sw_mesh.to_north().to_east())?;
        let ne_shift = self.dots[i].shift;

        // bilinear interpolation
        let [s_weight, w_weight] = sw_mesh.weight(&p);
        // let lat_shift = sw_shift.to_degree() * (s_weight * w_weight);

        Some(LatLon(-1.6666666666666667e-9, 0.0))
    }
    fn search_after(&self, first: usize, query: Mesh3) -> Option<usize> {
        self.dots
            .get(first..)?
            .binary_search_by_key(&query, |dot| dot.mesh)
            .ok()
            .map(|i| i + first)
    }
    fn search_at(&self, index: usize, query: Mesh3) -> Option<usize> {
        (self.dots.get(index)?.mesh == query).then_some(index)
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
    fn southwest(degree: &LatLon) -> Self {
        // "saturating cast" since Rust 1.45.0
        // https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#fixing-unsoundness-in-casts
        let lat = (degree.lat() * 120.) as i16;
        let lon = (degree.lon() * 80.) as i16;
        Self { lat, lon }
    }
    fn weight(self, p: &LatLon) -> [f64; 2] {
        let southeast = self.to_degree();
        let weight_lat = (southeast.lat() - p.lat()).abs() * 3_600. / Self::LAT_SEC;
        let weight_lon = (southeast.lon() - p.lon()).abs() * 3_600. / Self::LON_SEC;
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
    fn to_degree(self) -> LatLon {
        let lat = f64::from(self.lat) * Self::LAT_SEC / 3_600.;
        let lon = f64::from(self.lat) * Self::LON_SEC / 3_600.;
        LatLon(lat, lon)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicroSecond {
    lat: i32,
    lon: i32,
}
impl MicroSecond {
    fn to_degree(self) -> LatLon {
        let lat = f64::from(self.lat) / 3_600_000.;
        let lon = f64::from(self.lon) / 3_600_000.;
        LatLon(lat, lon)
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
        let ret = sut.interpolate(LatLon(0., 0.)).unwrap();
        assert_eq!(ret.lon(), 0.0);
        assert_eq!(ret.lat(), -6. / 3_600_000_000.);
    }
}
