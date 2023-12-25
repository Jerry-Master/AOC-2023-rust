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
    let mut boxes: Vec<Vec<(String, u32)>> = vec![Default::default();256];
    for step in steps {
        let tmp_hash = hash(&step);
        update(&mut boxes, tmp_hash, &step);
    }
    let mut res = 0;
    for (i, caja) in boxes.into_iter().enumerate() {
        for (j, element) in caja.into_iter().enumerate() {
            res += (i + 1) * (j + 1) * element.1 as usize;
        }
    }
    println!("{}", res);
    Ok(())
}


fn update(boxes: &mut Vec<Vec<(String, u32)>>, idx: usize, step: &String) {
    let name = step.split('-').nth(0).unwrap().split('=').nth(0).unwrap();
    if step.chars().last().unwrap() == '-' {
        delete(&mut boxes[idx], name);
    } else {
        let focal = step.chars().last().unwrap().to_digit(10).unwrap();
        insert_or_update(&mut boxes[idx], name, focal);
    }
}


fn delete(caja: &mut Vec<(String, u32)>, name: &str) {
    let mut aux = vec![];
    for element in caja.iter() {
        if element.0 != name {
            aux.push(element.clone());
        }
    }
    *caja = aux;
}


fn insert_or_update(caja: &mut Vec<(String, u32)>, name: &str, focal: u32) {
    let mut is_inside = false;
    for element in caja.iter_mut() {
        if element.0 == name {
            element.1 = focal;
            is_inside = true;
        }
    }
    if !is_inside {
        caja.push((String::from(name), focal));
    }
}



fn hash(seq: &String) -> usize {
    let mut res = 0;
    for char in seq.chars() {
        if char == '-' || char == '=' { break; }
        res += (char as u8) as usize;
        res *= 17;
        res %= 256;
    }
    res
}
