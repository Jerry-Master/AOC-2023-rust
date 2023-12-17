use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap;

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

    let mut map: Vec<Vec<u32>> = vec![];
    for line in reader.lines() {
        map.push(
            line?
                .chars()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
            );
    }
    // for row in &map {
    //     println!("{}", row.iter().map(|&x| char::from_digit(x, 10).unwrap()).collect::<String>());
    // }
    let dist = compute_distance(&map);
    println!("{}", dist);
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Tile {
    i: i32,
    j: i32,
    weight: u32,
    steps: u32,
    dir: Dir,
    heat_loss: u32,
}


// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.heat_loss.cmp(&self.heat_loss)
            .then_with(|| self.i.cmp(&other.i)
            .then_with(|| self.j.cmp(&other.j)
            .then_with(|| other.steps.cmp(&self.steps))
            .then_with(|| self.dir.cmp(&other.dir))))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn compute_distance(map: &Vec<Vec<u32>>) -> u32 {
    let mut queue = BinaryHeap::<Tile>::new();
    queue.push(Tile { i: 0, j: 1, weight: map[0][1], steps: 2, dir: Dir::Right, heat_loss: map[0][1] });
    queue.push(Tile { i: 1, j: 0, weight: map[1][0], steps: 2, dir: Dir::Down, heat_loss: map[1][0] });
    let mut dists: Vec<Vec<Vec<Vec<Option<u32>>>>> = vec![vec![vec![vec![None; 3]; 4]; map[0].len()]; map.len()];
    while let Some(tile) = queue.pop() {
        let idx = match tile.dir {
            Dir::Up => 0,
            Dir::Down => 1,
            Dir::Right => 2,
            Dir::Left => 3,
        };
        if let Some(tentative_heat_loss) = dists[tile.i as usize][tile.j as usize][idx][tile.steps as usize - 1] {
            if tentative_heat_loss <= tile.heat_loss {
                continue;
            }
        }
        dists[tile.i as usize][tile.j as usize][idx][tile.steps as usize - 1] = Some(tile.heat_loss);
        if tile.i == map.len() as i32 - 1 && tile.j == map[0].len() as i32 - 1 {
            return tile.heat_loss;
        }
        if tile.steps < 3 {
            let (i, j) = advance(tile.i, tile.j, tile.dir);
            if check(i, j, map) {
                queue.push(Tile {
                    i: i, j: j, weight: map[i as usize][j as usize],
                    steps: tile.steps + 1, dir: tile.dir,
                    heat_loss: tile.heat_loss + map[i as usize][j as usize]
                });
            }
        } 
        let (dir1, dir2) = match tile.dir {
            Dir::Up => (Dir::Left, Dir::Right),
            Dir::Down => (Dir::Left, Dir::Right),
            Dir::Left => (Dir::Up, Dir::Down),
            Dir::Right => (Dir::Up, Dir::Down),
        };
        let (i1, j1) = advance(tile.i, tile.j, dir1);
        if check(i1, j1, map) {
            queue.push(Tile {
                i: i1, j: j1, weight: map[i1 as usize][j1 as usize],
                steps: 1, dir: dir1,
                heat_loss: tile.heat_loss + map[i1 as usize][j1 as usize]
            });
        }
        let (i2, j2) = advance(tile.i, tile.j, dir2);
        if check(i2, j2, map) {
            queue.push(Tile { 
                i: i2, j: j2, weight: map[i2 as usize][j2 as usize],
                steps: 1, dir: dir2,
                heat_loss: tile.heat_loss + map[i2 as usize][j2 as usize]
            });
        }
    }
    panic!();
}


fn check(i: i32, j: i32, map: &Vec<Vec<u32>>) -> bool {
    return i >= 0 && j >= 0 && i < map.len() as i32 && j < map[0].len() as i32;
}


fn advance(i: i32, j: i32, dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Down => (i + 1, j),
        Dir::Left => (i, j - 1),
        Dir::Right => (i, j + 1),
        Dir::Up => (i - 1, j),
    }
}
