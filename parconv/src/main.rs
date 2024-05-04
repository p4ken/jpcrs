use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let stem = env::args().nth(1).expect("argument needed");
    let par_path = format!("{stem}.par");
    let rs_path = format!("{stem}.rs");
    let par_file = BufReader::new(File::open(par_path).unwrap());
    let mut rs_file = BufWriter::new(File::create(rs_path).unwrap());
    par_file
        .lines()
        .map(Result::unwrap)
        .skip_while(|line| line != "MeshCode   dB(sec)   dL(sec)")
        .skip(1)
        .map(convert_line)
        .for_each(|line| writeln!(rs_file, "{}", line).unwrap());
}

fn convert_line(line: String) -> String {
    line
}
