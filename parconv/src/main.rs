use std::{collections::BTreeSet, fmt::Display, io, str::FromStr};

use anyhow::{ensure, Context};

fn main() {
    io::stdin()
        .lines()
        .map(Result::unwrap)
        .skip_while(|line| line != "MeshCode   dB(sec)   dL(sec)")
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect::<BTreeSet<Record>>()
        .into_iter()
        .for_each(|record| println!("{}", record));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Record([i64; 4]);
impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(mut line: &str) -> anyhow::Result<Self> {
        fn parse_until(line: &mut &str, end: usize) -> anyhow::Result<i64> {
            ensure!(line.len() >= end);
            let (number, rest) = (&line[..end], &line[end..]);
            *line = rest;
            number.trim_start().parse().map_err(Into::into)
        }

        // 一次メッシュ
        let mesh1_lat = parse_until(&mut line, 2)?;
        let mesh1_lon = parse_until(&mut line, 2).unwrap();
        // 二次メッシュ
        let mesh2_lat = parse_until(&mut line, 1).unwrap();
        let mesh2_lon = parse_until(&mut line, 1).unwrap();
        // 三次メッシュ
        let mesh3_lat = parse_until(&mut line, 1).unwrap();
        let mesh3_lon = parse_until(&mut line, 1).unwrap();
        // 0度の三次メッシュを0とする連番のグリッド番号
        let grid_lat = mesh1_lat * 80 + mesh2_lat * 10 + mesh3_lat;
        let grid_lon = mesh1_lon * 80 + mesh2_lon * 10 + mesh3_lon;

        // 緯度差
        let d_lat_integer = parse_until(&mut line, 4).unwrap();
        line = line.strip_prefix(".").context("expected .")?;
        let d_lat_decimal = parse_until(&mut line, 5).unwrap();
        // 緯度差
        let d_lon_integer = parse_until(&mut line, 4).unwrap();
        line = line.strip_prefix('.').context("expected .")?;
        let d_lon_decimal = parse_until(&mut line, 5).unwrap();
        // 緯度経度差のマイクロ秒表現
        let d_lat_us = d_lat_integer * 1_000_000 + d_lat_decimal * 10;
        let d_lon_us = d_lon_integer * 1_000_000 + d_lon_decimal * 10;

        Ok(Record([grid_lat, grid_lon, d_lat_us, d_lon_us]))
    }
}
impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        write!(f, "({}),", fields.join(" "))
    }
}
