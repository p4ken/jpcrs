use latlonjp::{Datum, Degree, Jgd2011, LatLon, TokyoDatum, Transform, Unit};

fn api_elements() {
    let bl = geo_types::Point::new(2.0, 1.0);
    let bl = LatLon(1.0, 2.0);

    // let degree = latlonjp::degree(1.0, 2.0); // ambiguous lat/lon
    // let degree = LatLon {
    //     lat: 1.0,
    //     lon: 1.0,
    //     unit: Unit::Degree,
    // }; // redundant than Degree
    let degree = Degree { lat: 1.0, lon: 2.0 };
    let degree = LatLon::with_degree(1.0, 2.0);
    let degree = Transform::degree(bl);

    // let tokyo = degree.as_tokyo(); // wrong "as"
    let tokyo = TokyoDatum::new(degree);
    // let tokyo = TokyoDatum { lat: 1.0, lon: 2.0 }; // ambiguous unit
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
    let tokyo = latlonjp::tokyo(degree);

    let transform = Transform::from_tokyo().to_jgd2000().to_jgd2011();

    let jgd2011 = tokyo.to_jgd2000().to_jgd2011();
    let jgd2011 = tokyo.to_jgd2011();
    // let jgd2011 = bl.transform(Datum::Tokyo, Datum::Jgd2011); // combinatorial explosion
    // let jgd2011 = bl.tokyo_to_jgd2011(); // combinatorial explosion

    let Degree { lat, lon } = jgd2011.degree();
    let LatLon(lat, lon) = jgd2011.degree();

    let Degree { lat, lon } = transform.transform(degree);
    let LatLon(lat, lon) = transform.degree(bl);
    let LatLon(lat, lon) = transform.degree();
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

    let Degree { lat, lon } = Transform::from_tokyo(Degree { lat: 1.0, lon: 2.0 }) // Transform<Tokyo<Degree>>
        .to_jgd2000() // Transform<Jgd2000<Degree>>
        .to_jgd2011() // Transform<Jgd2011<Degree>>
        .degree();

    let Degree { lat, lon } = Transform::from_tokyo() // Transform<Tokyo>
        .to_jgd2000() // Transform<Jgd2000>
        .to_jgd2011() // Transform<Jgd2011>
        .transform(Degree { lat: 1.0, lon: 2.0 });

    let LatLon(lat, lon) = Transform::degree(LatLon(1.0, 2.0))
        .from_tokyo()
        .to_jgd2000()
        .to_jgd2011()
        .degree();

    let LatLon(lat, lon) = Transform::from_tokyo()
        .to_jgd2000()
        .to_jgd2011()
        .degree(LatLon(1.0, 2.0));

    // let p = geo_types::Point::new(2.0, 1.0);
    // let _ = TokyoDatum::with_degree(p)
    //     .to_jgd2000()
    //     .to_jgd2011()
    //     .degree()
    //     .into();
    // let _ = Transform::tokyo(p).to_jgd2000().to_jgd2011();
}
