// 10秒 42MB
// const TKY2JGD: &[ParRecord] = include!("../parconv/TKY2JGD.in");

// 20MB
const TKY2JGD: &[u8] = include_bytes!("../parconv/TKY2JGD.in");

#[test]
fn b() {
    dbg!(TKY2JGD.len());
}
