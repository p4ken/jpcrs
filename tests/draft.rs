use geo::{Coord, Point};
use jgd::{Degree_, LatLon, LonLat};

fn api_usage() {
    // let JGD2011 { lat, lon } = TokyoDatum { lat: 1.0, lon: 2.0 }.to_jgd2000().to_jgd2011();

    // 単位よりも緯度経度の順序の方が間違えやすい
    let LatLon(lat, lon) = jgd::from_tokyo_(LatLon(1.0, 2.0))
        .to_jgd2000()
        .to_jgd2011()
        .inner();

    let p = Point::from([2.0, 1.0]);
    let Point(_) = jgd::from_tokyo(p.y(), p.x())
        .to_jgd2000()
        .to_jgd2011()
        .lonlat()
        .into();

    let Point(_) = jgd::from_tokyo_(LonLat::from(p.x_y()))
        .to_jgd2000()
        .to_jgd2011()
        .inner()
        .lonlat()
        .into();
}
