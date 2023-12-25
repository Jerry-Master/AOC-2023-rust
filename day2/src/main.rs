use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::cmp::max;

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

    let mut sum = 0;
    for line in reader.lines() {
        let power = analyze(&line?);
        sum += power;
    }
    println!("{}", sum);
    Ok(())
}


fn analyze(line: &String) -> u32 {
    let game: Vec<&str> = line.split(": ").collect();
    let mut rm = 0;
    let mut gm = 0;
    let mut bm = 0;
    for play in game[1].split("; "){
        let (r, g, b) = count_play(play);
        rm = max(r, rm);
        gm = max(g, gm);
        bm = max(b, bm);
    }
    rm * gm * bm
}


fn count_play(play: &str) -> (u32, u32, u32) {
    let mut r = 0; let mut g = 0; let mut b = 0;
    for ball in play.split(", ") {
        let split: Vec<_> = ball.split(' ').collect();
        let num = split[0].parse::<u32>().unwrap();
        let color = split[1];
        if color == "red" {
            r = num;
        } else if color == "green" {
            g = num;
        } else {
            b = num;
        }
    }
    (r, g, b)
}