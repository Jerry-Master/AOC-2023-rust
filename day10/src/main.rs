use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec;
use clap::Parser;
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

enum Dir {
    North,
    East,
    West,
    South,
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = vec![];
    let mut m = 0;
    for line in reader.lines() {
        let mut row: Vec<char> = line?.chars().collect();
        if map.len() == 0 {
            m = row.len();
            map.push(vec!['.'; m + 2]);
        }
        map.push(vec!['.']);
        let n = map.len();
        map[n-1].append(&mut row);
        map[n-1].push('.');
    }
    map.push(vec!['.'; m + 2]);
    // println!("{:?}", map);
    let start = get_start(&map);
    let loop_len = get_loop_len(&map, &start);
    println!("{}", loop_len / 2);
    Ok(())
}


fn get_loop_len(map: &Vec<Vec<char>>, start: &(usize, usize)) -> u32 {
    let mut max_len: u32 = 0;
    for dir in [Dir::North, Dir::East, Dir::West, Dir::South] {
        let mut len = 1;
        let mut vis: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
        // println!("{:?}", vis);
        let mut u = start.clone();
        u = avance(&u, dir);
        vis[start.0][start.1] = true;
        while u != *start {
            // println!("{:?}", u);
            if map[u.0][u.1] == '.' { len = 0; break; }
            vis[u.0][u.1] = true;
            u = get_next(&map, &vis, &u);
            len += 1;
        }
        max_len = max(max_len, len);
    }
    return max_len;
}


fn get_next(map: &Vec<Vec<char>>, vis: &Vec<Vec<bool>>, u: &(usize, usize)) -> (usize, usize) {
    let pipe = map[u.0][u.1];
    let (u1, u2);
    match pipe {
        '|' => {
            u1 = avance(u, Dir::North);
            u2 = avance(u, Dir::South);
        },
        '-' => {
            u1 = avance(u, Dir::East);
            u2 = avance(u, Dir::West);
        },
        'L' => {
            u1 = avance(u, Dir::North);
            u2 = avance(u, Dir::East);
        },
        'J' => {
            u1 = avance(u, Dir::North);
            u2 = avance(u, Dir::West);
        },
        '7' => {
            u1 = avance(u, Dir::West);
            u2 = avance(u, Dir::South);
        },
        'F' => {
            u1 = avance(u, Dir::East);
            u2 = avance(u, Dir::South);
        },
        _ => panic!(),
    };
    if !vis[u1.0][u1.1] { return u1; }
    if !vis[u2.0][u2.1] { return u2; }
    if map[u1.0][u1.1] == 'S' { return u1; }
    if map[u2.0][u2.1] == 'S' { return u2; }
    panic!();
}


fn avance(u: &(usize, usize), dir: Dir) -> (usize, usize) {
    match dir {
        Dir::North => (u.0-1, u.1),
        Dir::West => (u.0, u.1-1),
        Dir::East => (u.0, u.1+1),
        Dir::South => (u.0+1, u.1),
    }
}


fn get_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 'S' {
                return (i, j);
            }
        }
    }
    return (0, 0);
}
