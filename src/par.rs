//! 座標変換パラメータ

// 10秒 42MB
// const TKY2JGD: &[ParRecord] = include!("../parconv/TKY2JGD.in");

// 20MB
const TKY2JGD: &[u8] = include_bytes!("../par/TKY2JGD.in");

// 22MB
// pub const TKY2JGD: &[u8] = include_bytes!("../par/TKY2JGD.in_");

// 22MB
// pub const TKY2JGD: &str = include_str!("../par/TKY2JGD.in_");

// 24MB
// pub const TKY2JGD: &[u8] = include_bytes!("../par/TKY2JGD.par");

#[test]
fn b() {
    dbg!(TKY2JGD.len());
}

struct Record(i16, i16, i32, i32);
