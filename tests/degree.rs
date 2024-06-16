use jgd::LatLon;

#[test]
fn from_d() {
    let sut = LatLon::new(1.0, 2.0);
    assert_eq!(sut.lat(), 1.0);
    assert_eq!(sut.lon(), 2.0)
}

#[test]
fn from_s() {
    let sut = LatLon::from_secs(3_600, 7_200);
    assert_eq!(sut.lat(), 1.0);
    assert_eq!(sut.lon(), 2.0)
}

#[test]
fn from_ms() {
    let sut = LatLon::from_milli_secs(3_600_000, 7_200_000);
    assert_eq!(sut.lat(), 1.0);
    assert_eq!(sut.lon(), 2.0)
}

#[test]
fn from_us() {
    let sut = LatLon::from_micro_secs(3_600_000_000., 7_200_000_000.);
    assert_eq!(sut.lat(), 1.0);
    assert_eq!(sut.lon(), 2.0)
}

#[test]
fn from_dms() {
    let sut = LatLon::from_dms((1, 6, 36), (2, 30, 0));
    assert_eq!(sut.lat(), 1.11);
    assert_eq!(sut.lon(), 2.50)
}
