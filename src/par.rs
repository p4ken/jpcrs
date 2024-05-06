//! 座標変換パラメータ

// 10秒 42MB
// const TKY2JGD: &[ParRecord] = include!("../parconv/TKY2JGD.in");

// 20MB
pub const TKY2JGD: &[u8] = include_bytes!("../par/TKY2JGD.in");

// 20MB
// pub const TKY2JGD: &str = include_str!("../par/TKY2JGD.in");

// 24MB
// pub const TKY2JGD: &str = include_str!("../par/TKY2JGD.par");

#[test]
fn last() {
    let r = Record::from_binary(&TKY2JGD[12 * 392322..]);
    assert_eq!(r.0, 5463);
    assert_eq!(r.1, 3356);
    assert_eq!(r.2, 00787532);
    assert_eq!(r.3, -1399561);
}

#[derive(Debug)]
struct Record(i16, i16, i32, i32);
impl Record {
    fn from_binary(bin: &[u8]) -> Self {
        Record(
            i16::from_le_bytes(bin[0..2].try_into().unwrap()),
            i16::from_le_bytes(bin[2..4].try_into().unwrap()),
            i32::from_le_bytes(bin[4..8].try_into().unwrap()),
            i32::from_le_bytes(bin[8..12].try_into().unwrap()),
        )
    }
}
