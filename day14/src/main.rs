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
    for line in reader.lines() {
        board.push(line?.chars().collect());
    }
    move_upward(&mut board);
    // for row in board {
    //     println!("{:?}", row.into_iter().collect::<String>());
    // }
    let res = count_load(&board);
    println!("{}", res);
    Ok(())
}


fn move_upward(board: &mut Vec<Vec<char>>) {
    let board_t = transpose(board);
    *board = transpose(
        &board_t
        .into_iter()
        .map(
            |x| {
                x
                    .into_iter()
                    .collect::<String>()
                    .split('#')
                    .map(|y| {
                        (y.chars().into_iter().filter(|&z| z == 'O').count(), y.len())
                    })
                    .map(|(x, y)| {
                        let left = vec!['O'; x].into_iter().collect::<String>();
                        let right = vec!['.'; y - x].into_iter().collect::<String>();
                        left + &right
                    })
                    .collect::<Vec<String>>()
                    .join("#")
                    .chars()
                    .collect::<Vec<char>>()
            }
        )
        .collect::<Vec<Vec<char>>>()
    );
}


fn count_load(board: &Vec<Vec<char>>) -> u32 {
    board
        .into_iter()
        .enumerate()
        .map(|(i, row)| (row.into_iter().filter(|&&x| x == 'O').count() * (board.len() - i)) as u32)
        .sum()
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