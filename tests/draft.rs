mod datumjp {
    pub fn tokyo<T>(_: T) -> TokyoDatum {}
    pub fn tokyo_to_jgd2000() -> JGD2000 {}
    pub fn tky2jgd() -> Transform {}
}

use datumjp::{
    Datum, Degree, Jgd2011, LatLon, TokyoDatum, Transform, Unit, JGD2011, TOKYO, TOKYO_TO_JGD2000,
    XY,
};

fn api_elements() {
    let bl = geo_types::Point::new(2.0, 1.0);
    let bl = LatLon(1.0, 2.0);
    let xy = XY(1.0, 2.0);

    // let degree = latlonjp::degree(1.0, 2.0); // ambiguous lat/lon
    // let degree = LatLon {
    //     lat: 1.0,
    //     lon: 1.0,
    //     unit: Unit::Degree,
    // }; // redundant than Degree
    let degree = Degree { lat: 1.0, lon: 2.0 };
    let degree = LatLon::with_degree(1.0, 2.0);
    let degree = Transform::degree(bl);

    let tokyo = degree.as_tokyo();
    let tokyo = TokyoDatum(degree);
    let tokyo = TokyoDatum::new(degree);
    let tokyo = TokyoDatum { lat: 1.0, lon: 2.0 };
    // let tokyo = LatLon {
    //     lat: 1.0,
    //     lon: 1.0,
    //     unit: Unit::Degree,
    //     datum: Datum::Tokyo, // just for input. output is trivial
    // };
    let tokyo = TokyoDatum {
        lat: 1.0,
        lon: 1.0,
        unit: Unit::Degree,
    };
    let tokyo = TokyoDatum::with_degree(bl);
    let tokyo = Transform::from_tokyo(degree);
    let tokyo = datumjp::tokyo(degree);

    let transform = Transform::from_tokyo().to_jgd2000().to_jgd2011();
    let transform = TOKYO.to_jgd2000().to_jgd2011();
    let transform = datumjp::tokyo_to_jgd2000().to_jgd2011();
    let transform = TOKYO_TO_JGD2000.to_jgd2011();
    let transform = datumjp::tky2jgd().tohokutaiheiyouoki().inverse();

    let jgd2011 = tokyo.to_jgd2000().to_jgd2011();
    let jgd2011 = tokyo.to_jgd2011();
    // let jgd2011 = bl.transform(Datum::Tokyo, Datum::Jgd2011); // combinatorial explosion
    // let jgd2011 = bl.tokyo_to_jgd2011(); // combinatorial explosion
    let jgd2011 = degree.transform(&transform);

    let JGD2011 { lat, lon } = jgd2011;
    let Degree { lat, lon } = jgd2011.degree();
    let LatLon(lat, lon) = jgd2011.degree();

    let Degree { lat, lon } = transform.transform(degree);
    let LatLon(lat, lon) = transform.degree(bl); // only for degree -> degree
    let LatLon(lat, lon) = transform.degree();
}

fn api_usage() {
    // let LatLon(lat, lon) = LatLon(1.0, 2.0)
    //     .as_tokyo()
    //     .to_jgd2000()
    //     .to_jgd2011()
    //     .latlon();

    let LatLon(lat, lon) = TokyoDatum::with_degree(LatLon(1.0, 2.0))
        .to_jgd2000()
        .to_jgd2011()
        .degree();

    let JGD2011 { lat, lon } = TokyoDatum { lat: 1.0, lon: 2.0 }.to_jgd2000().to_jgd2011();

    let LatLon(lat, lon) = TokyoDatum(LatLon(1.0, 2.0)).to_jgd2000().to_jgd2011().0;

    let LatLon(lat, lon) = Transform::from_tokyo(LatLon(1.0, 2.0))
        .to_jgd2000()
        .to_jgd2011()
        .transform();

    let LatLon(lat, lon) = Transform::from_tokyo()
        .to_jgd2000()
        .to_jgd2011()
        .transform(LatLon(1.0, 2.0));

    let p = geo_types::Point::new(2.0, 1.0);
}
