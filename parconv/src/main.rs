use std::{
    collections::BTreeSet,
    convert::Infallible,
    fmt::Display,
    io::{self, BufRead},
    str::{self, FromStr},
};

use anyhow::{ensure, Context};

fn main() {
    io::stdin()
        .lock()
        .split(b'\n')
        .map(|stdin| {
            let mut line = stdin.expect("stdin must be valid");
            assert_eq!(line.pop(), Some(b'\r'), "input must be CRLF");
            line
        })
        .skip_while(|line| line != b"MeshCode   dB(sec)   dL(sec)")
        .skip(1) // header
        .map(|line| {
            str::from_utf8(&line)
                .expect("body must be ASCII")
                .parse()
                .unwrap()
        })
        .collect::<BTreeSet<Record>>() // sort
        .into_iter()
        .for_each(|record| println!("{}", record));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Record(i64, i64, i64, i64);
impl FromStr for Record {
    type Err = Infallible;

    fn from_str(mut line: &str) -> Result<Record, Self::Err> {
        fn parse_number(line: &mut &str, end: usize) -> anyhow::Result<i64> {
            ensure!(line.len() >= end);
            let (number, rest) = (&line[..end], &line[end..]);
            *line = rest;
            number
                .trim_start()
                .parse()
                .context("failed to parse number")
        }

        fn parse_meshcode(line: &mut &str, chunk: usize) -> anyhow::Result<(i64, i64)> {
            Ok((parse_number(line, chunk)?, parse_number(line, chunk)?))
        }

        fn parse_difference(line: &mut &str) -> anyhow::Result<i64> {
            let d_integer = parse_number(line, 4)?;
            *line = line.strip_prefix(".").context("expected decimal point")?;
            let d_decimal = parse_number(line, 5)?;

            // マイクロ秒表現
            Ok(d_integer * 1_000_000 + d_decimal * 10)
        }

        let (mesh1_lat, mesh1_lon) = parse_meshcode(&mut line, 2).expect("1st mesh");
        let (mesh2_lat, mesh2_lon) = parse_meshcode(&mut line, 1).expect("2nd mesh");
        let (mesh3_lat, mesh3_lon) = parse_meshcode(&mut line, 1).expect("3rd mesh");

        // 0度の三次メッシュを0とする連番のグリッド番号
        let grid_lat = mesh1_lat * 80 + mesh2_lat * 10 + mesh3_lat;
        let grid_lon = mesh1_lon * 80 + mesh2_lon * 10 + mesh3_lon;

        let d_lat_us = parse_difference(&mut line).expect("dB(sec)");
        let d_lon_us = parse_difference(&mut line).expect("dL(sec)");

        Ok(Record(grid_lat, grid_lon, d_lat_us, d_lon_us))
    }
}
impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{},{}),", self.0, self.1, self.2, self.3)
    }
}
