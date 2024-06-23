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
                .parse::<Record>()
                .unwrap()
        })
        // sort all records, since lines 378632 onwards of TKY2JGD.par are not sorted
        .collect::<BTreeSet<_>>()
        .into_iter()
        .inspect(|record| eprintln!("{}", record))
        .for_each(|record| {
            io::stdout()
                .write_all(&record.to_binary())
                .expect("stdout must be valid")
        })
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Record(i16, i16, i32, i32);
impl FromStr for Record {
    type Err = Infallible;

    fn from_str(mut line: &str) -> Result<Record, Self::Err> {
        fn parse_number(line: &mut &str, end: usize) -> anyhow::Result<f64> {
            ensure!(line.len() >= end);
            let (number, rest) = (&line[..end], &line[end..]);
            *line = rest;
            number
                .trim_start()
                .parse()
                .context("failed to parse number")
        }

        fn parse_meshcode(line: &mut &str, chunk: usize) -> anyhow::Result<(i64, i64)> {
            let lat = parse_number(line, chunk)? as i64;
            let lon = parse_number(line, chunk)? as i64;
            Ok((lat, lon))
        }

        let (mesh1_lat, mesh1_lon) = parse_meshcode(&mut line, 2).expect("1st mesh");
        let (mesh2_lat, mesh2_lon) = parse_meshcode(&mut line, 1).expect("2nd mesh");
        let (mesh3_lat, mesh3_lon) = parse_meshcode(&mut line, 1).expect("3rd mesh");

        // Serial number of 3rd mesh grids starting from 0 degree
        fn to_grid_index(mesh1: i64, mesh2: i64, mesh3: i64) -> anyhow::Result<i16> {
            (mesh1 * 80 + mesh2 * 10 + mesh3)
                .try_into()
                .context("grid overflowed")
        }
        let index_lat = to_grid_index(mesh1_lat, mesh2_lat, mesh3_lat).expect("lat");
        let index_lon = to_grid_index(mesh1_lon + 100, mesh2_lon, mesh3_lon).expect("lon");

        fn parse_usec(line: &mut &str) -> anyhow::Result<i32> {
            let us = (parse_number(line, 10)? * 1_000_000.) as i32;
            ensure!(us.abs() < i32::MAX);
            Ok(us)
        }

        let d_lat_us = parse_usec(&mut line).expect("dB(sec)");
        let d_lon_us = parse_usec(&mut line).expect("dL(sec)");

        Ok(Record(index_lat, index_lon, d_lat_us, d_lon_us))
    }
}
impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:05},{:05},{:09},{:09}", self.0, self.1, self.2, self.3);
        assert_eq!(s.len(), 31, "{}", s);
        f.write_str(&s)
    }
}
impl Record {
    fn to_binary(&self) -> [u8; 12] {
        let mut buf = [0; 12];
        buf[0..2].copy_from_slice(&self.0.to_le_bytes());
        buf[2..4].copy_from_slice(&self.1.to_le_bytes());
        buf[4..8].copy_from_slice(&self.2.to_le_bytes());
        buf[8..12].copy_from_slice(&self.3.to_le_bytes());
        buf
    }
}
