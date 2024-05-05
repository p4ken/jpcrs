pub struct Degree(f64);

// pub struct MicroDegree(i32); // 8% 22cm

pub struct MilliSecond(i32); // 30% 60mm

// pub struct MicroSecond(i64); // 0% 0.06mm

pub struct NanoSecond(i64); // 0.007% 0mm

// pub struct Degree {
//     int: i16,
//     nanos: i32,
// }

// NanoSecondの方がよい
// pub struct Second {
//     second: i32, // 0.03%
//     micro: i32,  // 0.04%
// }

/// 緯経度
pub struct BL<T> {
    /// 緯度
    lat: T,

    /// 経度
    lon: T,
}
#[cfg(feature = "geo-types")]
impl From<geo_types::Coord> for BL<Degree> {
    fn from(coord: geo_types::Coord) -> Self {
        BL {
            lat: Degree(coord.y),
            lon: Degree(coord.x),
        }
    }
}
