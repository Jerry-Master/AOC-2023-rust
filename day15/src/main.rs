use std::fs::File;
use std::io::{self, prelude::*, BufReader};
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


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut steps = Vec::<String>::new();
    for line in reader.lines() {
        steps.append(&mut line?.split(',')
            .into_iter()
            .map(|x| String::from(x))
            .collect::<Vec<String>>());
    }
    let mut res = 0;
    for step in steps {
        res += hash(&step);
    }
    println!("{}", res);
    Ok(())
}


fn hash(seq: &String) -> u32 {
    let mut res = 0;
    for char in seq.chars() {
        res += (char as u8) as u32;
        res *= 17;
        res %= 256;
    }
    res
}
