use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::collections::HashMap;

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
    let initial_board = board.clone();
    let mut rep = HashMap::<Vec<Vec<char>>, u32>::new();
    rep.insert(board.clone(), 1);
    for _ in 0..1000 {
        move_cycle(&mut board);
        let prev_len = rep.len();
        rep.entry(board.clone()).and_modify(|counter| *counter += 1).or_insert(1);
        if prev_len == rep.len() { break; }
    }
    let big_cycle = rep.len();
    for _ in 0..big_cycle {
        move_cycle(&mut board);
        rep.entry(board.clone()).and_modify(|counter| *counter += 1).or_insert(1);
    }
    rep = rep.into_iter().filter(|(_, v)| *v > 1).collect();
    let offset = big_cycle - rep.len();
    let big_cycle = rep.len();
    let residue = (1000000000 - offset) % big_cycle;
    board = initial_board;
    for _ in 0..offset + residue {
        move_cycle(&mut board);
    }
    let res = count_load(&board);
    println!("{}", res);
    Ok(())
}


fn move_cycle(board: &mut Vec<Vec<char>>) {
    move_north(board);
    move_west(board);
    move_south(board);
    move_east(board);
}


fn move_north(board: &mut Vec<Vec<char>>) {
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


fn move_south(board: &mut Vec<Vec<char>>) {
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
                        let left = vec!['.'; y - x].into_iter().collect::<String>();
                        let right = vec!['O'; x].into_iter().collect::<String>();
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


fn move_west(board: &mut Vec<Vec<char>>) {
    let board_t = board.clone();
    *board = 
        board_t
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
        .collect::<Vec<Vec<char>>>();
}


fn move_east(board: &mut Vec<Vec<char>>) {
    let board_t = board.clone();
    *board = 
        board_t
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
                        let left = vec!['.'; y - x].into_iter().collect::<String>();
                        let right = vec!['O'; x].into_iter().collect::<String>();
                        left + &right
                    })
                    .collect::<Vec<String>>()
                    .join("#")
                    .chars()
                    .collect::<Vec<char>>()
            }
        )
        .collect::<Vec<Vec<char>>>();
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