use geo_types::Point;
use jgd::{LatLon, TokyoDatum, JGD2011, XY};

fn api_usage() {
    let JGD2011 { lat, lon } = TokyoDatum { lat: 1.0, lon: 2.0 }.to_jgd2000().to_jgd2011();

    let LatLon(lat, lon) = jgd::from_tokyo(LatLon(1.0, 2.0))
        .to_jgd2000()
        .to_jgd2011()
        .latlon();

    let p = Point::new(2.0, 1.0);
    // let Point(_) = TokyoDatum::with_xy(p).to_jgd2000().to_jgd2011().xy().into();
    let Point(_) = jgd::from_tokyo(p).to_jgd2000().to_jgd2011().xy().into();
}
