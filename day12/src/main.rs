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
 
 
fn count_damaged(line: &String) -> u32 {
    let mut text = line
        .split_ascii_whitespace()
        .nth(0)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let nums = line
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    // println!("{:?}", nums);
    let res = count_ways(&mut text, &nums);
    // println!("{:?}", text);
    return res;
}
 
fn count_ways(text: &mut Vec<char>, nums: &Vec<u32>) -> u32 {
    count_ways_brute_force(text, nums, 0)
}
 
fn count_ways_brute_force(text: &mut Vec<char>, nums: &Vec<u32>, i: usize) -> u32 {
    if i == text.len() {
        return match is_valid(text, nums) {
            true => 1,
            false => 0,
        };
    }
    if !is_valid_partial(text, nums, i) { 
        // println!("{:?}, {:?}, {}", text, nums, i);
        return 0; 
    }
    if text[i] == '?' {
        text[i] = '.';
        let dot_count = count_ways_brute_force(text, nums, i+1);
        text[i] = '#';
        let hashtag_count = count_ways_brute_force(text, nums, i+1);
        text[i] = '?';
        return dot_count + hashtag_count;
    } else {
        return count_ways_brute_force(text, nums, i+1);
    }
}


fn is_valid_partial(text: &Vec<char>, nums: &Vec<u32>, i: usize) -> bool {
    if i == 0 { return true; }
    let pred_nums = text[..i]
        .into_iter()
        .collect::<String>()
        .replace('.', " ")
        .split_whitespace()
        .map(|x| x.chars().count() as u32)
        .collect::<Vec<u32>>();
    if pred_nums.len() <= 1 { return true; }
    if pred_nums.len() > nums.len() { return false; }
    for i in 0..pred_nums.len() - 1 {
        if pred_nums[i] != nums[i] { return false; }
    }
    // println!("valid: {:?}, {:?}", text, nums);
    return true;
}
 
 
fn is_valid(text: &Vec<char>, nums: &Vec<u32>) -> bool {
    let pred_nums = text
        .into_iter()
        .collect::<String>()
        .replace('.', " ")
        .split_whitespace()
        .map(|x| x.chars().count() as u32)
        .collect::<Vec<u32>>();   
    if pred_nums.len() != nums.len() { return false; }
    for i in 0..nums.len() {
        if pred_nums[i] != nums[i] { return false; }
    }
    // println!("valid: {:?}, {:?}", text, nums);
    return true;
}