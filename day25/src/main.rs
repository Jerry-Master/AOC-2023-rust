use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
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

    /// Output file
    #[arg(short, long = "output-path")]
    output_path: String,
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let graph = read(&args)?;
    save(graph, args)?;  // Use Gephi to visualize and count subcomponents
    Ok(())
}


fn read(args: &Args) -> io::Result<HashMap<String, Vec<String>>> {
    let file = File::open(args.input_path.clone())?;
    let reader = BufReader::new(file);
    
    let mut graph = HashMap::<String, Vec<String>>::new();
    for line in reader.lines() {
        let line = line?;
        let mut it = line.split(": ");
        let key = String::from(it.next().unwrap());
        let mut conn = it.next().unwrap().split_whitespace().map(|x| String::from(x)).collect::<Vec<String>>();
        
        // for child in conn.iter() {
        //     graph.entry(child.clone()).or_insert(vec![]).push(key.clone());
        // }
        graph.entry(key).or_insert(vec![]).append(&mut conn);
    }
    return Ok(graph)
}


fn save(graph: HashMap<String, Vec<String>>, args: Args) -> io::Result<()> {
    let f = File::create(args.output_path).expect("Unable to create file");
    let mut out = BufWriter::new(f);
    out.write_fmt(format_args!("graph\n[\n"))?;
    for node in graph.keys() {
        out.write_fmt(format_args!("  node\n  [\n    id {}\n  ]\n", node))?;
    }
    for node in graph.keys() {
        for other in graph[node].iter() {
            out.write_fmt(format_args!("  edge\n  [\n    source {}\n    target {}\n  ]\n", node, other))?;
        }
    }
    out.write_fmt(format_args!("]\n"))?;
    Ok(())
}
