use std::{
    collections::BTreeSet,
    convert::Infallible,
    fmt::Display,
    io::{self, BufRead, Write},
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
        .for_each(|record| {
            io::stdout()
                .write_all(&record.to_be_bytes())
                .expect("stdout must be valid")
        })
    // .for_each(|record| println!("{}", record))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Record(i16, i16, i32, i32);
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

        let (mesh1_lat, mesh1_lon) = parse_meshcode(&mut line, 2).expect("1st mesh");
        let (mesh2_lat, mesh2_lon) = parse_meshcode(&mut line, 1).expect("2nd mesh");
        let (mesh3_lat, mesh3_lon) = parse_meshcode(&mut line, 1).expect("3rd mesh");

        // 0度の三次メッシュを0とする連番のグリッド番号
        fn to_grid(mesh1: i64, mesh2: i64, mesh3: i64) -> anyhow::Result<i16> {
            (mesh1 * 80 + mesh2 * 10 + mesh3)
                .try_into()
                .context("grid overflowed")
        }
        let grid_lat = to_grid(mesh1_lat, mesh2_lat, mesh3_lat).expect("lat");
        let grid_lon = to_grid(mesh1_lon, mesh2_lon, mesh3_lon).expect("lon");

        fn parse_diff(line: &mut &str) -> anyhow::Result<i32> {
            let d_integer = parse_number(line, 4)?;
            *line = line.strip_prefix(".").context("expected decimal point")?;
            let d_decimal = parse_number(line, 5)?;

            // マイクロ秒表現
            (d_integer * 1_000_000 + d_decimal * 10)
                .try_into()
                .context("diff overflowed")
        }

        let d_lat_us = parse_diff(&mut line).expect("dB(sec)");
        let d_lon_us = parse_diff(&mut line).expect("dL(sec)");

        Ok(Record(grid_lat, grid_lon, d_lat_us, d_lon_us))
    }
}
impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}{:04}{:09}{:09}", self.0, self.1, self.2, self.3)
    }
}
impl Record {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0
            .to_be_bytes()
            .into_iter()
            .chain(self.1.to_be_bytes().into_iter())
            .chain(self.2.to_be_bytes().into_iter())
            .chain(self.3.to_be_bytes().into_iter())
            .collect()
    }
}
