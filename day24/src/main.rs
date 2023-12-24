use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
struct Args {
    /// Input file
    #[arg(short, long = "input-path")]
    input_path: String,
}


#[derive(Clone, Copy, Debug)]
struct Hail {
    px: i128,
    py: i128,
    pz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}


impl Hail {
    fn intersect_2d(a: Hail, b: Hail) -> bool {
        let mut det = b.vx * a.vy - a.vx * b.vy;
        if det == 0 { return false; }
        let sgn;
        if det > 0 {
            sgn = 1;
        } else {
            sgn = -1;
            det = -det;
        }
        let lambda = (b.vy * (a.px - b.px) - b.vx * (a.py - b.py)) * sgn;
        let mu = (a.vy * (a.px - b.px) - a.vx * (a.py - b.py)) * sgn;
        if mu <= 0 || lambda <= 0 { return false; }
        let limits = (200000000000000, 400000000000000);
        return limits.0 * det <= a.px * det + a.vx * lambda
            && a.px * det + a.vx * lambda <= limits.1 * det
            && limits.0 * det <= a.py * det + a.vy * lambda
            && a.py * det + a.vy * lambda <= limits.1 * det;
    }
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut hails = Vec::<Hail>::new();
    for line in reader.lines() {
        hails.push(parse(line?));
    }
    let mut res = 0;
    for i in 0..hails.len() { 
        for j in i+1..hails.len() {
            if Hail::intersect_2d(hails[i], hails[j]) { res += 1; }
        }
    }
    println!("{}", res);
    Ok(())
}


fn parse(line: String) -> Hail {
    let mut it = line.split(" @ ");
    let mut left_it = it.next().unwrap().split(',');
    let mut right_it = it.next().unwrap().split(',');
    Hail { 
        px: left_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        py: left_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        pz: left_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        vx: right_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        vy: right_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        vz: right_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
    }
}