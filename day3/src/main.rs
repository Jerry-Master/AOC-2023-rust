use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec;
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

    let mut board: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        insert(&mut board, &line?);
    }
    let sum = add_part_numbers(&board);
    println!("{}", sum);
    Ok(())
}


fn insert(board: &mut Vec<Vec<char>>, line: &str){
    let row = board.len();
    board.push(vec![]);
    for char in line.chars() {
        board[row].push(char);
    }
}


fn add_part_numbers(board: &Vec<Vec<char>>) -> u32{
    let mut gears: Vec<(usize, usize, u32)> = vec![];  // num, row, col of gear
    for (row_num, row) in board.iter().enumerate() {
        let mut curr_num = String::from("");
        let mut start_col = 0;
        for (i, value) in row.iter().enumerate() {
            if value.is_ascii_digit() {
                if curr_num.len() == 0 {
                    start_col = i;
                }
                curr_num.push(*value);
                if i == row.len()-1 {  // The case of a number right at the end of line
                    let (is_gear, row, col) = check_gear(board, row_num, start_col, i);
                    if is_gear {
                        gears.push((row, col, curr_num.parse::<u32>().unwrap()));
                    }
                }
            } else if curr_num.len() > 0 {
                let (is_gear, row, col) = check_gear(board, row_num, start_col, i-1);
                if is_gear{
                    gears.push((row, col, curr_num.parse::<u32>().unwrap()));
                }
                curr_num = String::from("");
            }
        }
    }
    gears.sort_unstable();
    let mut num_equal = 0;
    let mut sum_ratios = 0;
    for i in 1..gears.len() {
        if gears[i-1].0 == gears[i].0 && gears[i-1].1 == gears[i].1 {
            num_equal += 1;
            if i == gears.len()-1 && num_equal == 1 {
                sum_ratios += gears[i].2 * gears[i-1].2;
            }
        } else  {
            if num_equal == 1 {
                sum_ratios += gears[i-1].2 * gears[i-2].2;
            }
            num_equal = 0;
        }
    }
    return sum_ratios;
}

fn check_gear(board: &Vec<Vec<char>>, row_num: usize, start_col: usize, end_col: usize) -> (bool, usize, usize) {
    let mut l_row = 0;
    if row_num > 0 {
        l_row = row_num - 1;
    }
    let mut l_col = 0;
    if start_col > 0 {
        l_col = start_col - 1;
    }
    for i in l_row..row_num+2 {
        for j in l_col..end_col+2 {
            if i == row_num && j != l_col && j != end_col + 1 {
                continue;
            }
            if is_valid(board, i, j) && board[i][j] == '*' {
                return (true, i, j);
            }
        }
    }
    (false, 0, 0)
}


fn is_valid(board: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    return i < board.len() && j < board[0].len();
}