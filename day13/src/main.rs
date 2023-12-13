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

    let mut board = Vec::<Vec<char>>::new();
    let mut res: u64 = 0;
    for line in reader.lines() {
        let line = line?;
        if line == "" {
            res += count_col_mirrors(&board);  // Count to the left
            res += 100 * count_row_mirrors(&board);  // Count above
            board = Vec::<Vec<char>>::new();
        } else {
            board.push(line.chars().into_iter().collect());
        }
    }
    res += count_col_mirrors(&board);  // Count to the left
    res += 100 * count_row_mirrors(&board);  // Count above
    println!("{}", res);
    Ok(())
}


fn count_row_mirrors(board: &Vec<Vec<char>>) -> u64 {
    for i in 0..board.len()-1 {
        if count_reflected_differences(board, i) {
            return i as u64 + 1;
        }
    }
    0
}


fn count_reflected_differences(board: &Vec<Vec<char>>, i: usize) -> bool {
    for ii in (0..i+1).rev() {
        if i+ii+1 < board.len() && board[i+ii+1] != board[i-ii] {
            return false;
        }        
    }
    true
}


fn count_col_mirrors(board: &Vec<Vec<char>>) -> u64 {
    count_row_mirrors(&transpose(board))
}


fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}