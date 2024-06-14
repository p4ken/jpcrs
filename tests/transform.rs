#[cfg(all(feature = "tky2jgd", feature = "patchjgd"))]
#[test]
fn tokyo_to_jgd2011() {
    let (lat, lon) = jgd::from_tokyo(35.0, 135.0)
        .to_jgd2000()
        .to_jgd2011()
        .lat_lon();
}
