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

    let mut res = 0;
    for line in reader.lines() {
        let mut nums = line?
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        res += compute_next(&mut nums);
    }
    println!("{}", res);
    Ok(())
}


fn compute_next(nums: &mut Vec<i32>) -> i32 {
    let next_diff = nums;
    let mut first_values = vec![];
    let mut sign = 1;
    while !all_zeros(next_diff){
        first_values.push(next_diff[0] * sign);
        sign *= -1;
        let diffs: Vec<i32> = next_diff
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        *next_diff = diffs;
    }
    return first_values
        .into_iter()
        .fold(0, |acc, x| acc + x);
}


fn all_zeros(vec: &Vec<i32>) -> bool {
    return vec
        .into_iter()
        .filter(|&&x| x == 0)
        .count() == vec.len();
}
