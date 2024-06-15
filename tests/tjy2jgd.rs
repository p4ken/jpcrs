use approx::assert_abs_diff_eq;
use jgd::{LatLon, Tokyo};

// const MILLI_SECOND_IN_DEGREES: f64 = 2.77778e-7;
const MILLI_METER_IN_DEGREES: f64 = 0.000000009;

fn assert_tky2jgd(tokyo: LatLon, jgd2000: LatLon) {
    let ret = Tokyo::new(tokyo).to_jgd2000().inner();
    assert_abs_diff_eq!(jgd2000.lat(), ret.lat(), epsilon = MILLI_METER_IN_DEGREES);
    assert_abs_diff_eq!(jgd2000.lon(), ret.lon(), epsilon = MILLI_METER_IN_DEGREES);
}

#[test]
fn muramatsu() {
    let tokyo = LatLon::from_dms((36, 27, 39.20500), (140, 35, 06.11100));
    let jgd2000 = LatLon::from_dms((36, 27, 50.58487), (140, 34, 54.10080));
    assert_tky2jgd(tokyo, jgd2000)
}
