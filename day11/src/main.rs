use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


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


fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut board = Vec::<Vec<char>>::new();
    for line in reader.lines() {
        let line = line?;
        board.push(line.chars().collect());
    }
    let dot_rows = get_dot_rows(&board);
    let dot_cols = get_dot_cols(&board);
    let galaxies = get_galaxies(&board);
    let distances = get_distances(&galaxies, &dot_rows, &dot_cols);
    let sum = distances.into_iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);
    Ok(())
}


fn get_dot_rows(board: &Vec<Vec<char>>) -> Vec<usize> {
    let mut res = Vec::<usize>::new();
    for (i, row) in board.into_iter().enumerate() {
        if all_dots(&row.into_iter().collect()) {
            res.push(i);
        }
    }
    return res;
}


fn get_dot_cols(board: &Vec<Vec<char>>) -> Vec<usize> {
    return get_dot_rows(&transpose(board));
}


fn get_distances(galaxies: &Vec<(usize, usize)>, dot_rows: &Vec<usize>, dot_cols: &Vec<usize>) -> Vec<usize> {
    let mut res = Vec::<usize>::new();
    for (i, &x1) in galaxies.into_iter().enumerate() {
        for (j, &x2) in galaxies.into_iter().enumerate() {
            if i < j {
                let cross_rows = count_crosses(x1.0, x2.0, dot_rows);
                let cross_cols = count_crosses(x1.1, x2.1, dot_cols);
                res.push(
                    (
                        (x1.0 as i32 - x2.0 as i32).abs() 
                        + (x1.1 as i32 - x2.1 as i32).abs()
                        + cross_rows * 999999 + cross_cols * 999999
                    ) as usize);
            }
        }
    }
    return res;
}


fn count_crosses(x1: usize, x2: usize, list: &Vec<usize>) -> i32 {
    if x1 == x2 { return 0; }
    if x1 > x2 { return count_crosses(x2, x1, list); }
    list.into_iter().filter(|&&x| x1 < x && x < x2).count() as i32
}


fn get_galaxies(board: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut res = Vec::<(usize, usize)>::new();
    for (i, row) in board.into_iter().enumerate() {
        for (j, value) in row.into_iter().enumerate() {
            if *value == '#' {
                res.push((i, j));
            }
        }
    }
    return res;
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


fn all_dots(line: &String) -> bool {
    let num_dots = line
        .chars()
        .filter(|&x| x == '.')
        .count();
    num_dots == line.len()
}
