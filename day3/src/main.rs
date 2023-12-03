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
    let mut sum = 0;
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
                    if check_symbol(board, row_num, start_col, i){
                        sum += curr_num.parse::<u32>().unwrap();
                    }
                }
            } else if curr_num.len() > 0 {
                if check_symbol(board, row_num, start_col, i-1){
                    sum += curr_num.parse::<u32>().unwrap();
                }
                curr_num = String::from("");
            }
        }
    }
    return sum;
}

fn check_symbol(board: &Vec<Vec<char>>, row_num: usize, start_col: usize, end_col: usize) -> bool {
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
            if is_valid(board, i, j) && board[i][j] != '.' && !board[i][j].is_ascii_digit() {
                return true;
            }
        }
    }
    false
}


fn is_valid(board: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    return i < board.len() && j < board[0].len();
}