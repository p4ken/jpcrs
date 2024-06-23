use jgd::LatLon;

#[test]
fn from_d() {
    let (lat, lon) = LatLon::from_degrees(1.0, 2.0).to_degrees();
    assert_eq!(lat, 1.0);
    assert_eq!(lon, 2.0)
}

#[test]
fn from_s() {
    let (lat, lon) = LatLon::from_secs(3_600, 7_200).to_degrees();
    assert_eq!(lat, 1.0);
    assert_eq!(lon, 2.0);
}

#[test]
fn from_ms() {
    let (lat, lon) = LatLon::from_milli_secs(3_600_000, 7_200_000).to_degrees();
    assert_eq!(lat, 1.0);
    assert_eq!(lon, 2.0);
}

#[test]
fn from_us() {
    let (lat, lon) = LatLon::from_micro_secs(3_600_000_000., 7_200_000_000.).to_degrees();
    assert_eq!(lat, 1.0);
    assert_eq!(lon, 2.0);
}

#[test]
fn from_dms() {
    let (lat, lon) = LatLon::from_dms((1, 6, 36), (2, 30, 0)).to_degrees();
    assert_eq!(lat, 1.11);
    assert_eq!(lon, 2.50);
}
