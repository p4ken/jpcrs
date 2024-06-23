//! 国土地理院によるオリジナルの PatchJGD と比較するテスト。
#![cfg(feature = "patchjgd")]

use approx::assert_abs_diff_eq;

/// 許容誤差: ±1mm
const MM_IN_DEGREES: f64 = 0.000000009;

#[test]
fn sendai() {
    let (lat, lon) = jgd::from_jgd2000(38.26, 140.87).to_jgd2011().into();
    assert_abs_diff_eq!(lat, 38.259991997, epsilon = MM_IN_DEGREES);
    assert_abs_diff_eq!(lon, 140.870036378, epsilon = MM_IN_DEGREES);
}
