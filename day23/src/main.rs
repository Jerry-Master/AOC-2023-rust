use std::fs::File;
use std::io::{self, prelude::*, BufReader};
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


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut map = Vec::<Vec<char>>::new();
    let mut start = None;
    for line in reader.lines() {
        map.push(line?.chars().collect());
        if start == None {
            start = match map[0]
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x == '.')
                .next() {
                    Some((i, _)) => Some(i as isize),
                    None => None,
                };
        }
    }
    let end = match map[map.len() - 1]
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x == '.')
                .next() {
                    Some((i, _)) => i as isize,
                    None => panic!(),
                };
    let mut dp: Vec<Vec<Option<u32>>> = vec![vec![None; map[0].len()]; map.len()];
    // println!("{:?}, {:?}", start, end);
    let mut vis = vec![vec![false; map[0].len()]; map.len()];
    let res = compute_maximum(&map, &mut dp, &mut vis, (0, start.unwrap()), (map.len() as isize - 1, end));
    println!("{}", res);
    Ok(())
}


enum Dir {
    Up,
    Down,
    Left,
    Right,
}


fn compute_maximum(map: &Vec<Vec<char>>, dp: &mut Vec<Vec<Option<u32>>>, vis: &mut Vec<Vec<bool>>, start: (isize, isize), end: (isize, isize)) -> u32 {
    if start == end { return 0; }
    if let Some(res) = dp[end.0 as usize][end.1 as usize] { return res; }
    vis[end.0 as usize][end.1 as usize] = true;
    // println!("{:?}", end);
    let mut res = 0;
    let next = advance(end, Dir::Up);
    if !out_of_bounds(map, next) && !vis[next.0 as usize][next.1 as usize] &&
        (map[next.0 as usize][next.1 as usize] == '.' || map[next.0 as usize][next.1 as usize] == 'v') {
        res = max(res, compute_maximum(map, dp, vis, start, next) + 1);
    }

    let next = advance(end, Dir::Down);
    if !out_of_bounds(map, next) && !vis[next.0 as usize][next.1 as usize] &&
        (map[next.0 as usize][next.1 as usize] == '.' || map[next.0 as usize][next.1 as usize] == '^') {
        res = max(res, compute_maximum(map, dp, vis, start, next) + 1);
    }

    let next = advance(end, Dir::Left);
    if !out_of_bounds(map, next) && !vis[next.0 as usize][next.1 as usize] &&
        (map[next.0 as usize][next.1 as usize] == '.' || map[next.0 as usize][next.1 as usize] == '>') {
        res = max(res, compute_maximum(map, dp, vis, start, next) + 1);
    }

    let next = advance(end, Dir::Right);
    if !out_of_bounds(map, next) && !vis[next.0 as usize][next.1 as usize] &&
        (map[next.0 as usize][next.1 as usize] == '.' || map[next.0 as usize][next.1 as usize] == '<') {
        res = max(res, compute_maximum(map, dp, vis, start, next) + 1);
    }
    dp[end.0 as usize][end.1 as usize] = Some(res);
    vis[end.0 as usize][end.1 as usize] = false;
    res
}


fn advance(pos: (isize, isize), dir: Dir) -> (isize, isize) {
    match dir {
        Dir::Up => (pos.0 - 1, pos.1),
        Dir::Down => (pos.0 + 1, pos.1),
        Dir::Left => (pos.0, pos.1 - 1),
        Dir::Right => (pos.0, pos.1 + 1),
    }
}


fn out_of_bounds(map: &Vec<Vec<char>>, end: (isize, isize)) -> bool {
    return end.0 < 0 || end.1 < 0 || end.0 >= map.len() as isize || end.1 >= map[0].len() as isize;
}
