use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::collections::HashSet;

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

    let mut points = 0;
    for line in reader.lines() {
        points += count_points(&line?);
    }
    println!("{}", points);
    Ok(())
}

fn count_points(line: &str) -> u32 {
    let mut win_nums: Vec<u32> = vec![];
    let mut own_nums = HashSet::new();
    read_input(line, &mut win_nums, &mut own_nums);
    let matches = count_matches(&win_nums, &own_nums);
    if matches == 0 {
        return 0;
    }
    return 1 << (matches - 1);
}


fn count_matches(vec: &Vec<u32>, set: &HashSet<u32>) -> u32 {
    let mut count = 0;
    for num in vec {
        if set.contains(num) {
            count += 1;
        }
    }
    return count;
}


fn read_input(line: &str, win_nums: &mut Vec<u32>, own_nums: &mut HashSet<u32>){
    let split: Vec<&str> = line.split(':').collect();
    let split2: Vec<&str> = split[1].split('|').collect();
    let winning = split2[0];
    let own = split2[1];
    for num_str in winning.split(' ') {
        if let Ok(num) = num_str.parse::<u32>(){
            win_nums.push(num);
        }
    }
    for num_str in own.split(' ') {
        if let Ok(num) = num_str.parse::<u32>(){
            own_nums.insert(num);
        }
    }
}
