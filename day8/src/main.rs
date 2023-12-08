use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::collections::HashMap;

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

    let mut graph = HashMap::<String, (String, String)>::new();
    let mut lines = reader.lines();
    let instructions = lines.next().unwrap()?.chars().collect::<Vec<char>>();
    let _ = lines.next().unwrap()?;
    while let Some(line) = lines.next() {
        let line = line?;
        *graph.entry(line[..3].to_string()).or_insert((String::new(), String::new())) = (line[7..10].to_string(), line[12..15].to_string());
    }
    let mut curr = String::from("AAA");
    let mut count = 0;
    while curr != "ZZZ" {
        let idx = (count % instructions.len()) as usize;
        curr = match instructions[idx] {
            'L' => graph[&curr].0.clone(),
            'R' => graph[&curr].1.clone(),
            _ => panic!(),
        };
        count += 1;
    }
    println!("{}", count);
    Ok(())
}
