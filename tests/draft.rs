#![cfg(feature = "default")]

use geo::{Coord, Line, MapCoords, Point};
use jgd::Tokyo;

fn api_usage() {
    // let JGD2011 { lat, lon } = TokyoDatum { lat: 1.0, lon: 2.0 }.to_jgd2000().to_jgd2011();

    // 単位よりも緯度経度の順序の方が間違えやすい
    let (lat, lon) = jgd::from_tokyo(1.0, 2.0)
        .to_jgd2000()
        .to_jgd2011()
        .lat_lon();

    let line = Line::new([0., 0.].into(), [2., 1.]);
    let line = line.map_coords(|Coord { x, y }| {
        let (y, x) = jgd::from_tokyo(y, x).to_jgd2000().to_jgd2011().lat_lon();
        Coord { x, y }
    });
}
