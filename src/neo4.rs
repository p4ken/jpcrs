pub fn usage() {
    // Tokyo::new(0., 1., DEGREES).to_jgd2000().lat_lon(DEGREES);

    // let (lat, lon) = LatLon::Seconds(1., 2.).from_tokyo().to_jgd2000().seconds();

    // let (lat, lon) = LatLon::Seconds(1., 2.)
    //     .tokyo_to_jgd2000()
    //     .jgd2011_to_jgd2000()
    //     .seconds();

    // LatLon(1., 2.) * SECONDS;

    from_tokyo(1., 2.).unit(MILLI_SECS);

    LatLon(1., 2.);
    LatLon::<MilliSec>(1., 2.);

    let (lat, lon) = Transform::from_tokyo()
        .to_jgd2000()
        .to_jgd2011()
        .milli_secs(1., 2.);

    Transform::from_tokyo()
        .to_jgd2000()
        .to_jgd2011()
        .transform(LatLon::from_milli_secs(1., 2.));

    jgd::transform(LatLon::from_secs(1., 2.))
        .from_tokyo()
        .to_jgd2000()
        .to_jgd2011()
        .degrees();
}
