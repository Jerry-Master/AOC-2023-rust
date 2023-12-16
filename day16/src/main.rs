use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::collections::VecDeque;
use std::cmp::max;

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

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Energized,
    MirrorDown,  // \
    MirrorUp,  // /
    SplitterVert,
    SplitterHor,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut board: Vec<Vec<Tile>> = vec![];
    for line in reader.lines() {
        board.push(
            line?
                .chars()
                .into_iter()
                .map(|x| { 
                    match x {
                        '.' => Tile::Empty,
                        '-' => Tile::SplitterHor,
                        '|' => Tile::SplitterVert,
                        '/' => Tile::MirrorUp,
                        '\\' => Tile::MirrorDown,
                        _ => panic!(),
                    }
                })
                .collect::<Vec<_>>()
        );
    }
    let mut res = 0;
    let num_rows = board.len();
    let num_cols = board[0].len();
    for i in 0..num_rows {
        // Left and Right
        let tmp = propagate(&mut board, (i as i32, 0, Dir::Right));
        res = max(res, tmp);
        let tmp = propagate(&mut board, (i as i32, (num_cols - 1) as i32, Dir::Left));
        res = max(res, tmp);
    }
    for j in 0..num_cols {
        // Top and Bottom
        let tmp = propagate(&mut board, (0, j as i32, Dir::Down));
        res = max(res, tmp);
        let tmp = propagate(&mut board, ((num_rows - 1) as i32, j as i32, Dir::Up));
        res = max(res, tmp);
    }
    // let res = propagate(&mut board, (0, 0, Dir::Right));
    show_board(&board);
    println!("{}", res);
    Ok(())
}


fn propagate(board: &mut Vec<Vec<Tile>>, start: (i32, i32, Dir)) -> u32 {
    let mut copy = vec![vec![vec![Dir::Left; 0]; board[0].len()]; board.len()];
    let mut rays = VecDeque::<(i32, i32, Dir)>::new();
    rays.push_back(start);
    while let Some(ray) = rays.pop_front() {
        let (i, j, mut dir) = ray;
        if i < 0 || j < 0 || i >= board.len() as i32 || j >= board[0].len() as i32 { continue; }
        let (i, j) = (i as usize, j as usize);
        if copy[i][j].contains(&dir) { continue; }
        copy[i][j].push(dir);
        match board[i][j] {
            Tile::Empty | Tile::Energized => {
                board[i][j] = Tile::Energized;
                let (i, j) = advance(i, j, dir);
                rays.push_back((i, j, dir));
            },
            Tile::MirrorUp => {
                dir = match dir {
                    Dir::Down => Dir::Left,
                    Dir::Up => Dir::Right,
                    Dir::Left=> Dir::Down,
                    Dir::Right => Dir::Up,
                };
                let (i, j) = advance(i, j, dir);
                rays.push_back((i, j, dir));
            },
            Tile::MirrorDown => {
                dir = match dir {
                    Dir::Down => Dir::Right,
                    Dir::Up => Dir::Left,
                    Dir::Left=> Dir::Up,
                    Dir::Right => Dir::Down,
                };
                let (i, j) = advance(i, j, dir);
                rays.push_back((i, j, dir));
            },
            Tile::SplitterHor => {
                match dir {
                    Dir::Down | Dir::Up => {
                        let (dir1, dir2) = (Dir::Left, Dir::Right);
                        let (i1, j1) = advance(i, j, dir1);
                        let (i2, j2) = advance(i, j, dir2);
                        rays.push_back((i1, j1, dir1));
                        rays.push_back((i2, j2, dir2));
                    },
                    Dir::Left | Dir::Right => {
                        let (i, j) = advance(i, j, dir);
                        rays.push_back((i, j, dir));
                    },
                };
            },
            Tile::SplitterVert => {
                match dir {
                    Dir::Left | Dir::Right => {
                        let (dir1, dir2) = (Dir::Up, Dir::Down);
                        let (i1, j1) = advance(i, j, dir1);
                        let (i2, j2) = advance(i, j, dir2);
                        rays.push_back((i1, j1, dir1));
                        rays.push_back((i2, j2, dir2));
                    },
                    Dir::Up | Dir::Down => {
                        let (i, j) = advance(i, j, dir);
                        rays.push_back((i, j, dir));
                    },
                };
            },
        }
    }
    return copy.iter().map(|row| {
        row.iter().filter(|value| value.len() > 0).count() as u32
    }).sum();
}


fn advance(i: usize, j: usize, dir: Dir) -> (i32, i32) {
    let (i, j) = (i as i32, j as i32);
    match dir {
        Dir::Down => (i + 1, j),
        Dir::Up => (i - 1, j),
        Dir::Left=> (i, j - 1),
        Dir::Right => (i, j + 1),
    }
}


fn show_board(board: &Vec<Vec<Tile>>) {
    for row in board {
        for tile in row {
            print!("{}", match tile {
                Tile::Empty => '.',
                Tile::SplitterHor => '-',
                Tile::SplitterVert => '|',
                Tile::MirrorUp => '/',
                Tile::MirrorDown => '\\',
                Tile::Energized => '#',
            });
        }
        println!()
    }
}
