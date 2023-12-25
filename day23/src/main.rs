use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::cmp::max;
use std::collections::{VecDeque, HashMap};

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
    let graph = construct_graph(&map, (0, start.unwrap()), (map.len() as isize - 1, end));
    // println!("{:?}", graph);
    let mut vis = vec![false; graph.len()];
    vis[0] = true;
    let mut res = 0;
    brute_force_count(&graph, &mut vis, 0, 1, 0, &mut res);
    println!("{}", res);
    Ok(())
}


fn brute_force_count(graph: &Vec<Vec<(usize, u32)>>, vis: &mut Vec<bool>, start: usize, end: usize, dist: u32, res: &mut u32) {
    if start == end { 
        *res = max(*res, dist);
    }
    for &(node, next_dist) in graph[start].iter() {
        if vis[node] { continue; }
        vis[node] = true;
        brute_force_count(graph, vis, node, end, dist + next_dist, res);
        vis[node] = false;
    }
}


fn construct_graph(map: &Vec<Vec<char>>, start: (isize, isize), end: (isize, isize)) -> Vec<Vec<(usize, u32)>> {
    // Find intersections
    let mut graph = vec![];
    let mut nodes = vec![];
    let mut nodes_dict = HashMap::<(isize, isize), usize>::new();
    nodes.push((start.0 as usize, start.1 as usize));
    nodes.push((end.0 as usize, end.1 as usize));
    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            if check_intersection(map, (i,j)) {
                nodes_dict.insert((i as isize, j as isize), nodes.len());
                nodes.push((i, j));
            }
        }
    }
    // println!("{:?}", nodes);
    // BFS from each intersection to the others
    for (i, node) in nodes.iter().enumerate() {
        let mut queue = VecDeque::<((usize, usize), u32)>::new();
        queue.push_back((*node, 0));
        let mut vis = vec![vec![false; map[0].len()]; map.len()];
        while let Some((pos, dist)) = queue.pop_front() {
            if vis[pos.0][pos.1] { continue; }
            vis[pos.0][pos.1] = true;
            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                let next = advance((pos.0 as isize, pos.1 as isize), dir);
                if !out_of_bounds(map, next) && map[next.0 as usize][next.1 as usize] != '#' {
                    if check_intersection(map, (next.0 as usize, next.1 as usize)) && !vis[next.0 as usize][next.1 as usize] {
                        if graph.len() <= i {
                            graph.push(vec![]);
                        } 
                        graph[i].push((nodes_dict[&next], dist + 1));
                    } else {
                        queue.push_front(((next.0 as usize, next.1 as usize), dist + 1));
                    }
                }
            }
        }
    }
    // Make undirected graph
    for i in 0..graph.len() {
        let mut adj = vec![];
        for &(j, dist) in graph[i].iter() {
            adj.push((j, dist));
        }
        'outer: for (j, dist) in adj {
            for node in graph[j].iter() {
                if node.0 == i {
                    continue 'outer;
                }
            }
            graph[j].push((i, dist));
        }
    }
    graph
}


fn check_intersection(map: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    if map[pos.0][pos.1] == '#' { return false; }
    let mut count = 0;
    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
        let next_pos = advance((pos.0 as isize, pos.1 as isize), dir);
        if !out_of_bounds(map, next_pos) && map[next_pos.0 as usize][next_pos.1 as usize] != '#' {
            count += 1;
        }
    }
    count > 2
}


enum Dir {
    Up,
    Down,
    Left,
    Right,
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
