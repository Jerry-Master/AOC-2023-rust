use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec;
use clap::Parser;
use std::collections::VecDeque;

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
    Up,
    Down,
    Left,
    Right,
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    // let mut map = Vec::<Vec<char>>::new();
    // for line in reader.lines() {
    //     let line = line?;
    //     map.push(
    //         line
    //          .chars()
    //          .collect()
    //     );
    // }
    // let iters = 3;
    // let start = get_start(&map, iters);
    // mirror(&mut map, iters);
    // walk(&mut map, start, 65+131+131+131);  // This gives the series of results: 3699, 33137, 91951, 180141 which form an ARIMA(1,2,0) with AR coeff=1
    // // for row in map.iter() {
    // //     println!("{}", row.iter().collect::<String>());
    // // }
    // let res = count(&map);
    
    // 26501365 = 202300 * 131 + 65
    let mut x0: u64 = 3699;
    let mut x1 = 33137;
    let mut x2 = 91951;
    let mut res = 0;
    for _ in 0..202300-2 {
        res = 3 * x2 - 3 * x1 + x0;
        x0 = x1; x1 = x2; x2 = res;
    }

    println!("{}", res);
    Ok(())
}


fn mirror(map: &mut Vec<Vec<char>>, iters: usize) {
    let mut new_map: Vec<Vec<char>> = vec![vec!['.'; map[0].len() * (2 * iters + 1)]; map.len() * (2 * iters + 1)];
    for i in 0..new_map.len() {
        for j in 0..new_map[0].len() {
            new_map[i][j] = map[i % map.len()][j % map[0].len()];
        }
    }
    *map = new_map;
}


fn walk(map: &mut Vec<Vec<char>>, start: (i64, i64), dist: i64) {
    let mut queue = VecDeque::<(i64, i64, i64)>::new();  // i, j, dist
    queue.push_back((start.0, start.1, 0));
    let mut curr_dist = 0;
    let mut vis: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    while let Some(curr) = queue.pop_front() {
        if curr.2 == dist + 1 { break; }
        if vis[curr.0 as usize][curr.1 as usize] { continue; }
        if curr.2 > curr_dist {
            curr_dist = curr.2;
            for i in 0..map.len() {
                for j in 0..map[0].len() {
                    if vis[i][j] {
                        map[i][j] = '.';
                        vis[i][j] = false;
                    }
                }
            }
        }
        map[curr.0 as usize][curr.1 as usize] = 'O';
        vis[curr.0 as usize][curr.1 as usize] = true;
        for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let next = advance(curr, dir);
            if check(next, map) {
                queue.push_back(next);
            }
        }
    }
}


fn check(next: (i64, i64, i64), map: &Vec<Vec<char>>) -> bool {
    return next.0 >= 0 && next.1 >= 0 && next.0 < map.len() as i64 && next.1 < map[0].len() as i64 && map[next.0 as usize][next.1 as usize] != '#';
}


fn advance(curr: (i64, i64, i64), dir: Dir) -> (i64, i64, i64) {
    match dir {
        Dir::Up => (curr.0 - 1, curr.1, curr.2 + 1),
        Dir::Down => (curr.0 + 1, curr.1, curr.2 + 1),
        Dir::Left => (curr.0, curr.1 - 1, curr.2 + 1),
        Dir::Right => (curr.0, curr.1 + 1, curr.2 + 1),
    }
}


fn count(map: &Vec<Vec<char>>) -> i64 {
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                res += 1;
            }
        }
    }
    // let res = map.iter().map(|row| row.iter().filter(|&&x| x == 'O').count() as i64).sum();
    res
}


fn get_start(map: &Vec<Vec<char>>, iters: usize) -> (i64, i64) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                return ((i + iters * map.len()) as i64, (j + iters * map[0].len()) as i64);
            }
        }
    }
    panic!()
}
