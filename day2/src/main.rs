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


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let (id, is_possible) = analyze(&line?);
        if is_possible {
            sum += id;
        }
    }
    println!("{}", sum);
    Ok(())
}


fn analyze(line: &String) -> (u32, bool) {
    let game: Vec<&str> = line.split(": ").collect();
    let id = get_id(game[0]);
    let mut is_possible = true;
    for play in game[1].split("; "){
        is_possible = is_possible && check_play(play);
        if !is_possible {
            return (id, false);
        }
    }
    (id, is_possible)
}


fn check_play(play: &str) -> bool {
    for ball in play.split(", ") {
        let split: Vec<_> = ball.split(' ').collect();
        let num = split[0].parse::<u32>().unwrap();
        let color = split[1];
        if !check_ball(num, color) {
            return false;
        }
    }
    true
}


fn check_ball(num: u32, color: &str) -> bool {
    if color == "red" {
        return num <= 12;
    }
    if color == "green" {
        return num <= 13;
    }
    return num <= 14;
}


fn get_id(text: &str) -> u32 {
    let text_id: Vec<_> = text.split(' ').collect();
    let text_id = text_id[text_id.len()-1];
    return text_id.parse::<u32>().unwrap();
}
