use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::cmp::{Ord, PartialOrd, Ordering, max};
use std::collections::HashSet;

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


#[derive(PartialEq, Eq, Debug)]
struct Brick {
    x0: u32,
    y0: u32,
    z0: u32,
    x1: u32,
    y1: u32,
    z1: u32,
}


impl Brick {
    fn supports(&self, other: &Self) -> bool {
        if self.z1 >= other.z0 { return false; }
        if self.x1 < other.x0 { return false; }
        if self.x0 > other.x1 { return false; }
        if self.y1 < other.y0 { return false; }
        if self.y0 > other.y1 { return false; }
        true
    }
}


impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z0.cmp(&other.z0)
    }
}
impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut bricks = Vec::<Brick>::new();
    for line in reader.lines() {
        let line = line?;
        bricks.push(parse(line));
    }
    bricks.sort();
    let (graph, inv_graph) = construct_tree(bricks);
    // println!("{:?}", graph);
    let res = count_removable(graph, inv_graph);
    println!("{}", res);
    Ok(())
}


fn count_removable(graph: Vec<HashSet<usize>>, inv_graph: Vec<HashSet<usize>>) -> u32 {
    let mut count = 0;
    for node_children in graph.iter() {
        let mut removable = true;
        for node_child in node_children {
            if inv_graph[*node_child].len() == 1 {
                removable = false;
                break;
            }
        }
        if removable {
            count += 1;
        }
    }
    count
}


fn construct_tree(bricks: Vec<Brick>) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    // println!("{:?}", bricks);
    let mut fallen_bricks: Vec<Brick> = vec![];
    for mut brick in bricks {
        let mut max_z1 = 0;
        for fallen_brick in fallen_bricks.iter() {
            if fallen_brick.supports(&brick) {
                max_z1 = max(max_z1, fallen_brick.z1);
            }
        }
        let diff = brick.z1 - brick.z0;
        brick.z0 = max_z1 + 1;
        brick.z1 = brick.z0 + diff;
        fallen_bricks.push(brick);
    }
    // println!("{:?}", fallen_bricks);


    let mut graph = vec![HashSet::new(); fallen_bricks.len()];
    let mut inv_graph = vec![HashSet::new(); fallen_bricks.len()];
    for i in 0..fallen_bricks.len() {
        for j in i+1..fallen_bricks.len() {
            if fallen_bricks[i].z1 + 1 != fallen_bricks[j].z0 { continue; }
            if fallen_bricks[i].supports(&fallen_bricks[j]) {
                graph[i].insert(j);
                inv_graph[j].insert(i);
            }
        }
    }
    (graph, inv_graph)
}


fn parse(line: String) -> Brick {
    let mut it = line.split('~');
    let mut start_it = it.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap());
    let mut end_it = it.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap());
    Brick { 
        x0: start_it.next().unwrap(), y0: start_it.next().unwrap(), z0: start_it.next().unwrap(),
        x1: end_it.next().unwrap(), y1: end_it.next().unwrap(), z1: end_it.next().unwrap()
    }
}
