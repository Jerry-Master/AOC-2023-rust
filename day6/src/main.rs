use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use itertools::izip;

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


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|x| x.unwrap());
    let times = lines
        .next().unwrap()
        .split(':')
        .nth(1).unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let dists = lines
        .next().unwrap()
        .split(':')
        .nth(1).unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut res = 1;
    for (t, d) in izip!(times, dists) {
        res *= count_ways(t as f64, d as f64);
    }
    println!("{}", res);
    Ok(())
}


fn count_ways(t: f64, d: f64) -> u32 {
    let upper = ((t + (t * t - 4. * d).sqrt()) / 2. - 0.000001).floor() as u32;
    let lower = ((t - (t * t - 4. * d).sqrt()) / 2. + 0.000001).ceil() as u32;
    return upper - lower + 1;
}