use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::cmp::{max, min};

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


#[derive(Debug, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instr {
    dir: Dir,
    length: usize,
    color: [u8; 3]
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut instructs = Vec::<Instr>::new();
    for line in reader.lines() {
        let line = line?;
        let mut words = line.split_whitespace();
        instructs.push(
            Instr {
                dir: match words.next().unwrap().chars().nth(0).unwrap() {
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    'U' => Dir::Up,
                    'D' => Dir::Down,
                    _ => panic!(),
                },
                length: words.next().unwrap().parse().unwrap(),
                color: parse_color(words.next().unwrap()),
            }
        )
    }
    // println!("{:#?}", instructs);
    let ((m, n), start) = get_shape(&instructs);  // Board of m x n
    println!("{}, {}", m, n);
    let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; m];
    draw(&mut board, &instructs, start);
    fill(&mut board);
    for row in &board {
        println!("{}", row.iter().collect::<String>());
    }
    let res = count(&board);
    println!("{}", res);
    Ok(())
}


fn fill(board: &mut Vec<Vec<char>>) {
    let mut copy = board.clone();
    for i in 0..board.len() {
        let mut is_in = false;
        let mut first_dir: Dir = Dir::Right;
        let mut last_dir: Dir = Dir::Right;
        for j in 0..(board[0].len() - 1) {
            if board[i][j] == '#' {
                if j == 0 || board[i][j - 1] == '.' {  // First encounter
                    if i > 0 && board[i - 1][j] == '#' {
                        first_dir = Dir::Up;
                    } 
                    if i < board.len() - 1 && board[i + 1][j] == '#' {
                        if first_dir == Dir::Up {
                            first_dir = Dir::Right;
                        } else {
                            first_dir = Dir::Down;
                        }
                    }
                }
                if board[i][j + 1] == '.' {  // Last encounter
                    if i > 0 && board[i - 1][j] == '#' {
                        last_dir = Dir::Up;
                    } 
                    if i < board.len() - 1 && board[i + 1][j] == '#' {
                        if last_dir == Dir::Up {
                            last_dir = Dir::Right;
                        } else {
                            last_dir = Dir::Down;
                        }
                    }
                    if first_dir != last_dir || last_dir == Dir::Right {
                        is_in = !is_in;
                    }
                    (first_dir, last_dir) = (Dir::Right, Dir::Right);
                }
            } else if is_in {
                copy[i][j] = '#';
            }
        }
    }
    *board = copy;
}


fn count(board: &Vec<Vec<char>>) -> u32 {
    board
        .iter()
        .map(
            |x| 
                x
                .iter()
                .filter(|&&x| x == '#')
                .count() as u32
        ).sum()
}


fn draw(board: &mut Vec<Vec<char>>, instructs: &Vec<Instr>, start: (usize, usize)) {
    let (mut i, mut j): (usize, usize) = start;
    for instr in instructs {
        for d in 0..instr.length {
            match instr.dir {
                Dir::Up => board[i - d][j] = '#',
                Dir::Down => board[i + d][j] = '#',
                Dir::Left => board[i][j - d] = '#',
                Dir::Right => board[i][j + d] = '#',
            };
            (i, j) = (i, j);
        }
        match instr.dir {
            Dir::Up => i = i - instr.length,
            Dir::Down => i = i + instr.length,
            Dir::Left => j = j - instr.length,
            Dir::Right => j = j + instr.length,
        };
    }
}


fn get_shape(instructs: &Vec<Instr>) -> ((usize, usize), (usize, usize)){
    let (mut m, mut n) = (1, 1);
    let (mut m_max, mut n_max) = (1, 1);
    let (mut m_min, mut n_min) = (1, 1);
    for instr in instructs {
        match instr.dir {
            Dir::Left => { n -= instr.length as i32; n_min = min(n, n_min) },
            Dir::Right => { n += instr.length as i32; n_max = max(n, n_max) },
            Dir::Down => { m += instr.length as i32; m_max = max(m, m_max) },
            Dir::Up => { m -= instr.length as i32; m_min = min(m, m_min) },
        }
    }
    (
        ((m_max - m_min + 1) as usize, (n_max - n_min + 1) as usize),
        ((1 - m_min) as usize, (1 - n_min) as usize)
    )
}


fn parse_color(hex: &str) -> [u8; 3] {
    [
        u8::from_str_radix(&hex[2..4], 16).unwrap(),
        u8::from_str_radix(&hex[4..6], 16).unwrap(),
        u8::from_str_radix(&hex[6..8], 16).unwrap(),    
    ]
}
