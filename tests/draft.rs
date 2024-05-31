use geo::{Coord, Point};

fn api_usage() {
    // let JGD2011 { lat, lon } = TokyoDatum { lat: 1.0, lon: 2.0 }.to_jgd2000().to_jgd2011();

    let [lon, lat] = jgd::from_tokyo([2.0, 1.0]).to_jgd2000().to_jgd2011().xy();

    let p = Coord::from([2.0, 1.0]);
    let Point(_) = jgd::from_tokyo(p.into())
        .to_jgd2000()
        .to_jgd2011()
        .xy()
        .into();
}
