use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec;
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

#[derive(Copy, Clone, Debug)]
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
    let start = get_start(&map);
    let (loop_dir1, loop_dir2) = get_loop_dir(&map, &start);
    clean_loop(&mut map, &start, loop_dir1);
    mark_start(&mut map, &start, loop_dir1, loop_dir2);
    mark_inside(&mut map);
    let res = count_inside(&map);
    for row in map {
        println!("{:?}", row.into_iter().collect::<String>());
    }
    println!("{}", res);
    Ok(())
}


fn count_inside(map: &Vec<Vec<char>>) -> u32 {
    map
        .iter()
        .map(|x| x.iter().filter(|&&y| y == 'I').count() as u32)
        .fold(0, |acc, x| acc + x)
}


fn mark_inside(map: &mut Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                map[i][j] = check_inside(map, &(i, j));
            }
        }
    }
}


fn check_inside(map: &Vec<Vec<char>>, pos: &(usize, usize)) -> char {
    let mut crosses = 0;
    let mut is_L = false;
    let mut is_J = false;
    let mut curr = pos.clone();
    while map[curr.0][curr.1] != 'I' && map[curr.0][curr.1] != 'O' {
        curr = avance(&curr, Dir::North);
        match map[curr.0][curr.1] {
            '-' => crosses += 1,
            'L' => is_L = true,
            'J' => is_J = true,
            '7' => {
                if is_L { crosses += 1; is_L = false; }
                else { is_J = false; }
            },
            'F' => {
                if is_J { crosses += 1; is_J = false; }
                else { is_L = false; }
            },
            _ => {},
        };
    }
    match crosses % 2 {
        0 => map[curr.0][curr.1],
        1 => match map[curr.0][curr.1] {
            'I' => 'O',
            'O' => 'I',
            _ => panic!(),
        },
        _ => panic!()
    }
}


fn mark_start(map: &mut Vec<Vec<char>>, start: &(usize, usize), loop_dir1: Dir, loop_dir2: Dir) {
    map[start.0][start.1] = match (
        loop_dir1, loop_dir2
    ) {
        (Dir::North, Dir::North) => '|',
        (Dir::North, Dir::East) => 'J',
        (Dir::North, Dir::West) => 'L',
        (Dir::South, Dir::South) => '|',
        (Dir::South, Dir::East) => '7',
        (Dir::South, Dir::West) => 'F',
        (Dir::East, Dir::East) => '-',
        (Dir::East, Dir::North) => 'F',
        (Dir::East, Dir::South) => 'L',
        (Dir::West, Dir::West) => '-',
        (Dir::West, Dir::North) => '7',
        (Dir::West, Dir::South) => 'J',
        _ => panic!(),
    };
}


fn clean_loop(map: &mut Vec<Vec<char>>, start: &(usize, usize), loop_dir: Dir) {
    let mut clean = vec![vec!['.'; map[0].len()]; map.len()];
    let mut vis: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    // println!("{:?}", vis);
    let mut u = start.clone();
    u = avance(&u, loop_dir);
    vis[start.0][start.1] = true;
    while u != *start {
        clean[u.0][u.1] = map[u.0][u.1];
        // println!("{:?}", u);
        if map[u.0][u.1] == '.' { panic!(); }
        vis[u.0][u.1] = true;
        (u, _) = get_next(&map, &vis, &u);
    }
    for i in 0..map.len() {
        clean[i][0] = 'O';
        clean[i][map[0].len()-1] = 'O';
    }
    for j in 0..map[0].len() {
        clean[0][j] = 'O';
        clean[map.len()-1][j] = 'O';
    }
    *map = clean;
}


fn get_loop_dir(map: &Vec<Vec<char>>, start: &(usize, usize)) -> (Dir, Dir) {
    let mut max_len: u32 = 0;
    let mut max_dir: Dir = Dir::North;
    let mut other_dir: Dir = Dir::North;
    for dir in [Dir::North, Dir::East, Dir::West, Dir::South] {
        let mut len = 1;
        let mut vis: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
        // println!("{:?}", vis);
        let mut u = start.clone();
        u = avance(&u, dir);
        vis[start.0][start.1] = true;
        let mut aux_dir: Dir = Dir::North;
        while u != *start {
            // println!("{:?}", u);
            if map[u.0][u.1] == '.' { len = 0; break; }
            vis[u.0][u.1] = true;
            (u, aux_dir) = get_next(&map, &vis, &u);
            len += 1;
        }
        if len > max_len {
            max_len = len;
            max_dir = dir;
            other_dir = aux_dir;
        }
    }
    return (max_dir, other_dir);
}


fn get_next(map: &Vec<Vec<char>>, vis: &Vec<Vec<bool>>, u: &(usize, usize)) -> ((usize, usize), Dir) {
    let pipe = map[u.0][u.1];
    let (dir1, dir2);
    match pipe {
        '|' => {
            dir1 = Dir::North;
            dir2 = Dir::South;
        },
        '-' => {
            dir1 = Dir::East;
            dir2 = Dir::West;
        },
        'L' => {
            dir1 = Dir::North;
            dir2 = Dir::East;
        },
        'J' => {
            dir1 = Dir::North;
            dir2 = Dir::West;
        },
        '7' => {
            dir1 = Dir::West;
            dir2 = Dir::South;
        },
        'F' => {
            dir1 = Dir::East;
            dir2 = Dir::South;
        },
        _ => panic!(),
    };
    let u1 = avance(u, dir1);
    let u2 = avance(u, dir2);
    if !vis[u1.0][u1.1] { return (u1, dir1); }
    if !vis[u2.0][u2.1] { return (u2, dir2); }
    if map[u1.0][u1.1] == 'S' { return (u1, dir1); }
    if map[u2.0][u2.1] == 'S' { return (u2, dir2); }
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
