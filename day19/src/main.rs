use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::collections::HashMap;
use std::cmp::Ordering;

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

#[derive(Debug)]
enum Part {
    X, M, A, S
}

#[derive(Debug)]
struct Cond {
    part: Part,
    op: Ordering,
    val: u32,
    left: String,
}


#[derive(Clone, Copy, Debug)]
struct Item {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut is_instr = true;
    let mut instrs = HashMap::<String, Vec<Cond>>::new();
    let mut res = 0;
    for line in reader.lines() {
        let line = line?;
        if line == "" { is_instr = false; }
        else if is_instr {
            let mut tokens = line.split('{');
            let id = String::from(tokens.next().unwrap());
            let conds = parse_cond(tokens.next().unwrap());
            instrs.insert(id, conds);
        } else {
            let item = parse_item(&line);
            if accepted(item, &instrs) {
                res += value(item);
            }
        }
    }
    println!("{}", res);
    Ok(())
}


fn parse_item(line: &String) -> Item {
    let mut it = line[1..line.len()-1].split(',');
    let x = it.next().unwrap().split('=').nth(1).unwrap().parse().unwrap();
    let m = it.next().unwrap().split('=').nth(1).unwrap().parse().unwrap();
    let a = it.next().unwrap().split('=').nth(1).unwrap().parse().unwrap();
    let s = it.next().unwrap().split('=').nth(1).unwrap().parse().unwrap();
    Item { x: x, m: m, a: a, s: s }
}

fn accepted(item: Item, instrs: &HashMap<String, Vec<Cond>>) -> bool {
    let mut curr_state = String::from("in");
    while curr_state != "R" && curr_state != "A" {
        let conds = &instrs[&curr_state];
        for cond in conds {
            let item_val = match cond.part {
                Part::X => item.x,
                Part::M => item.m,
                Part::A => item.a,
                Part::S => item.s,
            };
            match cond.op {
                Ordering::Equal => {
                    curr_state = cond.left.clone();
                    break;
                },
                Ordering::Less => {
                    if item_val < cond.val {
                        curr_state = cond.left.clone();
                        break;
                    }
                },
                Ordering::Greater => {
                    if item_val > cond.val {
                        curr_state = cond.left.clone();
                        break;
                    }
                },
            };
        }
    }
    curr_state == "A"
}

fn value(item: Item) -> u32 {
    item.x + item.m + item.a + item.s
}

fn parse_cond(txt: &str) -> Vec<Cond> {
    let txt = &txt[..txt.len()-1];
    let mut res = vec![];
    for instr in txt.split(',') {
        let (part, op, val, state);
        if instr.contains(':') {
            state = String::from(instr.split(':').nth(1).unwrap());
            let instr = instr.split(':').nth(0).unwrap();
            if instr.contains('>') {
                op = Ordering::Greater;
                part = match instr.split('>').nth(0).unwrap() {
                    "x" => Part::X,
                    "m" => Part::M,
                    "a" => Part::A,
                    "s" => Part::S,
                    _ => panic!(),
                };
                val = instr.split('>').nth(1).unwrap().parse().unwrap();
            } else {
                assert!(instr.contains('<'));
                op = Ordering::Less;
                part = match instr.split('<').nth(0).unwrap() {
                    "x" => Part::X,
                    "m" => Part::M,
                    "a" => Part::A,
                    "s" => Part::S,
                    _ => panic!(),
                };
                val = instr.split('<').nth(1).unwrap().parse().unwrap();
            }
        } else {
            op = Ordering::Equal;
            state = String::from(instr);
            val = 0;
            part = Part::X;
        }
        res.push(Cond { part: part, op: op, val: val, left: state})
    }
    res
}
