use geo::Point;
use jgd::{LatLon, LonLat};

fn api_usage() {
    // let JGD2011 { lat, lon } = TokyoDatum { lat: 1.0, lon: 2.0 }.to_jgd2000().to_jgd2011();

    // 単位よりも緯度経度の順序の方が間違えやすい
    let (lat, lon) = jgd::from_tokyo(LatLon(1.0, 2.0))
        .to_jgd2000()
        .to_jgd2011()
        .inner();

    let p = Point::from([2.0, 1.0]);
    let Point(_) = jgd::from_tokyo(LonLat::new(p))
        .to_jgd2000()
        .to_jgd2011()
        .inner()
        .into();

    // let Point(_) = jgd::from_tokyo(LatLon::from_lonlat(p))
    //     .to_jgd2000()
    //     .to_jgd2011()
    //     .lonlat()
    //     .into();
}
