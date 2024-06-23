//! projの実装と比較するテスト。

use approx::assert_abs_diff_eq;

/// 許容誤差: ±1mm
const MM_IN_DEGREES: f64 = 0.000000009;

#[test]
fn towgs84() {
    let (lat, lon) = jgd::from_tokyo97(35., 135.).to_jgd2000().into();
    assert_abs_diff_eq!(lat, 35.00319718, epsilon = MM_IN_DEGREES);
    assert_abs_diff_eq!(lon, 134.99720425, epsilon = MM_IN_DEGREES);
}

#[test]
fn towgs84_inverse() {
    let (lat, lon) = jgd::from_jgd2000(35., 135.).to_tokyo97().into();
    assert_abs_diff_eq!(lat, 34.99680236, epsilon = MM_IN_DEGREES);
    assert_abs_diff_eq!(lon, 135.00279591, epsilon = MM_IN_DEGREES);
}
