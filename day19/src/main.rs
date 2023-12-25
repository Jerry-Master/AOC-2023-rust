use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec;
use clap::Parser;
use std::collections::HashMap;
use std::cmp::{Ordering, max, min};

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
    val: u64,
    left: String,
}

#[derive(Clone, Copy, Debug)]
struct Interval {  // Inclusive on both ends
    l: u64,
    r: u64,
}


impl Default for Interval {
    fn default() -> Self {
        Interval {l: 1, r: 4000}
    }
}


impl Interval {
    fn intersect(a: Self, b: Self) -> Option<Self> {
        let l = max(a.l, b.l);
        let r = min(a.r, b.r);
        if l <= r { return Some(Interval {l: l, r: r}); }
        None
    }

    fn len(&self) -> u64 {
        return self.r - self.l + 1;
    }
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut instrs = HashMap::<String, Vec<Cond>>::new();
    for line in reader.lines() {
        let line = line?;
        if line == "" { break; }
        let mut tokens = line.split('{');
        let id = String::from(tokens.next().unwrap());
        let conds = parse_cond(tokens.next().unwrap());
        instrs.insert(id, conds);
    }
    let mut a_coords = find_accepted(&instrs);
    let parents = compute_parents(&instrs);
    let intervals = propagate_intervals(&instrs, &mut a_coords, &parents);
    let res = count_combinations(intervals);
    println!("{}", res);
    Ok(())
}


fn count_combinations(intervals: Vec<IntTuple>) -> u64 {
    let mut total = 0;
    for el in intervals {
        total += el.0.len() * el.1.len() * el.2.len() * el.3.len();
    }
    total
}

#[derive(Clone, Copy, Debug)]
struct IntTuple(Interval, Interval, Interval, Interval);
impl Default for IntTuple {
    fn default() -> Self {
        IntTuple(Interval::default(), Interval::default(), Interval::default(), Interval::default())
    }
}
impl IntTuple {
    fn intersect(a: Self, b: Self) -> Option<Self> {
        let x0 = match Interval::intersect(a.0, b.0) {
            Some(x0) => x0,
            None => return None,
        };
        let x1 = match Interval::intersect(a.1, b.1) {
            Some(x1) => x1,
            None => return None,
        };
        let x2 = match Interval::intersect(a.2, b.2) {
            Some(x2) => x2,
            None => return None,
        };
        let x3 = match Interval::intersect(a.3, b.3) {
            Some(x3) => x3,
            None => return None,
        };
        Some(IntTuple(x0, x1, x2, x3))
    }
}
fn propagate_intervals(instrs: &HashMap<String, Vec<Cond>>, a_coords: &mut Vec<(String, usize, IntTuple)>, parents: &HashMap<String, Vec<(String, usize)>>) -> Vec<IntTuple> {
    let mut res = vec![];
    let mut count = 0;
    while count < 100 && a_coords.len() > 0 {
        count += 1;
        let mut next_coords: Vec<(String, usize, IntTuple)> = vec![];
        'outer: for el in a_coords.iter() {
            let mut restriction = el.2;
            for i in 0..el.1 {
                let cond = &instrs[&el.0][i];
                let interval = match cond.op {
                    Ordering::Equal => panic!(),
                    Ordering::Less => Interval { l: cond.val, r: 4000 },
                    Ordering::Greater => Interval { l: 1, r: cond.val },
                };
                let interval_tuple = match cond.part {
                    Part::X => IntTuple(interval, Interval::default(), Interval::default(), Interval::default()),
                    Part::M => IntTuple(Interval::default(), interval, Interval::default(), Interval::default()),
                    Part::A => IntTuple(Interval::default(), Interval::default(), interval, Interval::default()),
                    Part::S => IntTuple(Interval::default(), Interval::default(), Interval::default(), interval),
                };
                if let Some(aux) = IntTuple::intersect(restriction, interval_tuple) {
                    restriction = aux;
                } else {
                    continue 'outer;
                }
            }
            let cond = &instrs[&el.0][el.1];
            let interval = match cond.op {
                Ordering::Equal => Interval::default(),
                Ordering::Greater => Interval { l: cond.val + 1, r: 4000 },
                Ordering::Less => Interval { l: 1, r: cond.val - 1 },
            };
            let interval_tuple = match cond.part {
                Part::X => IntTuple(interval, Interval::default(), Interval::default(), Interval::default()),
                Part::M => IntTuple(Interval::default(), interval, Interval::default(), Interval::default()),
                Part::A => IntTuple(Interval::default(), Interval::default(), interval, Interval::default()),
                Part::S => IntTuple(Interval::default(), Interval::default(), Interval::default(), interval),
            };
            if let Some(aux) = IntTuple::intersect(restriction, interval_tuple) {
                restriction = aux;
            } else {
                continue 'outer;
            }
            if el.0 == "in" { res.push(restriction); }
            else {
                for parent in &parents[&el.0] {
                    next_coords.push((parent.0.clone(), parent.1, restriction));
                }
            }
        }
        *a_coords = next_coords.clone();
    }
    res
}


fn compute_parents(instrs: &HashMap<String, Vec<Cond>>) -> HashMap<String, Vec<(String, usize)>> {
    let mut res = HashMap::<String, Vec<(String, usize)>>::new();
    for k in instrs.keys() {
        res.insert(k.clone(), vec![]);
    }
    res.insert(String::from("A"), vec![]);
    for (k, instr) in instrs {
        for i in 0..instr.len() {
            if instr[i].left == "R" { continue; }
            res.entry(instr[i].left.clone()).and_modify(|x| x.push((k.clone(), i)));
        }
    }
    return res;
}


fn find_accepted(instrs: &HashMap<String, Vec<Cond>>) -> Vec<(String, usize, IntTuple)> {
    let mut res = vec![];
    for (k, instr) in instrs {
        for i in 0..instr.len() {
            if instr[i].left == "A" {
                res.push((k.clone(), i, IntTuple::default()));
            }
        }
    }
    res
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
