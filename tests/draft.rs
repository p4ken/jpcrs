use latlonjp::{Datum, Degree, Jgd2011, LatLon, TokyoDatum, Unit};

fn api_elements() {
    let bl = geo_types::Point::new(2.0, 1.0);
    let bl = LatLon(1.0, 2.0);

    let degree = latlonjp::degree(1.0, 2.0);
    // let degree = LatLon {
    //     lat: 1.0,
    //     lon: 1.0,
    //     unit: Unit::Degree,
    // }; // redundant than Degree
    let degree = Degree { lat: 1.0, lon: 2.0 };
    let degree = LatLon::with_degree(1.0, 2.0);

    // let tokyo = degree.as_tokyo(); // wrong "as"
    // let tokyo = TokyoDatum::new(1.0, 2.0); // ambiguous unit and lat/lon
    // let tokyo = TokyoDatum(degree); // hard to have Into<Degree> like geo-types
    let tokyo = TokyoDatum::new(degree);
    // let tokyo = TokyoDatum { lat: 1.0, lon: 2.0 }; // ambiguous unit
    // let tokyo = TokyoDatum::with_degree(1.0, 2.0); // ambiguous lat/lon
    // let tokyo = LatLon {
    //     lat: 1.0,
    //     lon: 1.0,
    //     unit: Unit::Degree,
    //     datum: Datum::Tokyo, // just for input, as output is trivial
    // };
    let tokyo = TokyoDatum {
        lat: 1.0,
        lon: 1.0,
        unit: Unit::Degree,
    };
    let tokyo = TokyoDatum::with_degree(bl);

    let jgd2011 = tokyo.to_jgd2000().to_jgd2011();
    let jgd2011 = tokyo.to_jgd2011();
    // let jgd2011 = bl.transform(Datum::Tokyo, Datum::Jgd2011); // combinatorial explosion
    // let jgd2011 = bl.tokyo_to_jgd2011(); // combinatorial explosion

    // let (lat, lon) = jgd2011.lat_lon(); // ambiguous unit
    // let (lat, lon) = jgd2011.degree(); // ambiguous lat/lon
    // let Jgd2011 { lat, lon, unit } = jgd2011; // unit transform is needed
    let Degree { lat, lon } = jgd2011.degree();
    let LatLon(lat, lon) = jgd2011.degree();
}

fn api_usage() {
    let Degree { lat, lon } = TokyoDatum::new(Degree { lat: 1.0, lon: 2.0 })
        .to_jgd2000()
        .to_jgd2011()
        .degree();

    let LatLon(lat, lon) = TokyoDatum::with_degree(LatLon(1.0, 2.0))
        .to_jgd2000()
        .to_jgd2011()
        .degree();
}
