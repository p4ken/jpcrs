#![cfg(feature = "default")]

use geo::Point;
use jgd::{LatLon, LonLat, Tokyo};

fn api_usage() {
    // let JGD2011 { lat, lon } = TokyoDatum { lat: 1.0, lon: 2.0 }.to_jgd2000().to_jgd2011();

    // 単位よりも緯度経度の順序の方が間違えやすい
    let (lat, lon) = jgd::from_tokyo(1.0, 2.0)
        .to_jgd2000()
        .to_jgd2011()
        .lat_lon();

    let p = Point::from([2.0, 1.0]);
    let Point(_) = jgd::from_tokyo(p.y(), p.x())
        .to_jgd2000()
        .to_jgd2011()
        .lon_lat()
        .into();

    let Point(_) = Tokyo::with_lon_lat(p)
        .to_jgd2000()
        .to_jgd2011()
        .lon_lat()
        .into();
}
