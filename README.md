# jgd

Rust で日本の測地系を変換するライブラリ。

## Getting started

[![crates.io](https://img.shields.io/crates/v/jgd.svg)](https://crates.io/crates/jgd)

```shell
cargo add jgd
```

## Examples

旧日本測地系から世界測地系に変換する。

```rust
let (lat, lon) = jgd::from_tokyo(35.0, 135.0)
    .to_jgd2000()
    .to_jgd2011()
    .into();
```

## [API documentation](https://docs.rs/jgd/)

[![docs.rs](https://img.shields.io/badge/_-docs.rs-slategray?logo=docsdotrs)](https://docs.rs/jgd/)

## [MIT license](LICENSE.md)
