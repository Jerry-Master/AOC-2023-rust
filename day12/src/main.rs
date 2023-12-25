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
        res += count_damaged(&line?);
    }
    println!("{}", res);
    Ok(())
}
 
 
fn count_damaged(line: &String) -> u64 {
    let mut text = line
        .split_ascii_whitespace()
        .nth(0)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    // Repeat 5 times
    let orig = text.clone();
    for _ in 0..4 {
        text.push('?');
        text.append(&mut orig.clone());
    }
    let mut nums = line
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    // Repeat 5 times
    let orig = nums.clone();
    for _ in 0..4 {
        nums.append(&mut orig.clone());
    }
    let res = count_ways(&mut text, &nums);
    return res;
}
 
fn count_ways(text: &mut Vec<char>, nums: &Vec<usize>) -> u64 {
    let mut dp = vec![vec![None; nums.len()]; text.len()];
    count_ways_dp(text, nums, 0, 0, &mut dp)
}
 
fn count_ways_dp(text: &mut Vec<char>, nums: &Vec<usize>, i: usize, j: usize, dp: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if i >= text.len() {  // EOF
        if j >= nums.len() {  // All hashtags are in position
            return 1;
        }
        return 0;  // Some hashtags left
    }
    if j < nums.len() {  // Memoization
        if let Some(value) = dp[i][j] { return value; }
    }
    
    let res = match text[i] {
        '.' => count_ways_dp(text, nums, i+1, j, dp),
        '#' => count_ways_dp_hashtag(text, nums, i, j, dp),
        '?' => count_ways_dp(text, nums, i+1, j, dp) + count_ways_dp_hashtag(text, nums, i, j, dp),  // both . and #
        _ => panic!(),
    };
    if j < nums.len() {
        dp[i][j] = Some(res);
    }
    return res;
}


fn count_ways_dp_hashtag(text: &mut Vec<char>, nums: &Vec<usize>, i: usize, j: usize, dp: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if j >= nums.len() {  // No more hashtags
        return 0;
    }
    if i + nums[j] > text.len() {  // Not enough room for hashtags
        return 0;
    }
    for ii in i..i+nums[j] {
        if text[ii] == '.' {  // Impossible to fit enough consecutive hashtags
            return 0;
        }
    }
    // EOF
    if i + nums[j] == text.len() {
        if j == nums.len() - 1 {
            return 1;
        }
        return 0;
    }
    // Look that next character is not hashtag
    if text[i + nums[j]] == '#' { return 0; }
    count_ways_dp(text, nums, i + nums[j] + 1, j + 1, dp)
}

