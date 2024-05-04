//! 座標変換パラメータ

// メッシュ左下の緯度経度を秒単位で求める
// 緯度/30,経度/45が一意で連続のキーとなる
// fn to_grid_key(bl: BL) {}

// struct BL {
//     seconds: i32,
// }

struct Record(i16, i16, i32, i32);
