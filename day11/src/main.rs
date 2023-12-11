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
        if all_dots(&line) {
            board.push(line.chars().collect());
        }
        board.push(line.chars().collect());
    }
    duplicate_cols(&mut board);
    let galaxies = get_galaxies(&board);
    let distances = get_distances(&galaxies);
    let sum = distances.into_iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);
    // for line_vec in board {
    //     println!("{}", line_vec.into_iter().collect::<String>())
    // }
    Ok(())
}


fn get_distances(galaxies: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut res = Vec::<usize>::new();
    for (i, &x1) in galaxies.into_iter().enumerate() {
        for (j, &x2) in galaxies.into_iter().enumerate() {
            if i < j {
                res.push(((x1.0 as i32 - x2.0 as i32).abs() + (x1.1 as i32 - x2.1 as i32).abs()) as usize);
            }
        }
    }
    return res;
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


fn duplicate_cols(board: &mut Vec<Vec<char>>) {
    let transposed = transpose(board);
    let mut new = Vec::<Vec<char>>::new();
    for row in &transposed {
        if all_dots(&row.into_iter().collect()) {
            new.push(row.clone());
        }
        new.push(row.clone());
    }
    *board = transpose(&new);
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
