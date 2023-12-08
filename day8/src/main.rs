use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::collections::HashMap;
use itertools::izip;

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

    // Input read and parse
    let mut graph = HashMap::<String, (String, String)>::new();
    let mut lines = reader.lines();
    let instructions = lines.next().unwrap()?.chars().collect::<Vec<char>>();
    let _ = lines.next().unwrap()?;
    while let Some(line) = lines.next() {
        let line = line?;
        *graph.entry(line[..3].to_string()).or_insert((String::new(), String::new())) = (line[7..10].to_string(), line[12..15].to_string());
    }

    // Detect entry nodes
    let mut curr_states: Vec<_> = graph
        .keys()
        .filter(|x| x.ends_with('A'))
        .collect();
    println!("{}", curr_states.len());

    // Traversal
    let mut counts: Vec<u32> = vec![0; curr_states.len()];
    for (curr, count) in izip!(curr_states.iter_mut(), counts.iter_mut()){
        while !curr.ends_with('Z') {
            let idx = *count as usize % instructions.len();
            
                *curr = match instructions[idx] {
                    'L' => &mut &graph[*curr].0,
                    'R' => &mut &graph[*curr].1,
                    _ => panic!(),
                };
            *count += 1;
        }
    }
    let total = mcm(&counts);
    println!("{}", total);
    Ok(())
}


fn mcm(vec: &Vec<u32>) -> u64 {
    let mut mcm = vec[0] as u64;
    for i in 1..vec.len() {
        mcm = mcm_binary(mcm as u64, vec[i] as u64);
    }
    return mcm;
}

fn mcm_binary(a: u64, b: u64) -> u64 {
    return a * b / mcd_binary(a, b);
}

fn mcd_binary(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    return mcd_binary(b, a % b);
}
