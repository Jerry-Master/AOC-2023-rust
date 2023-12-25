use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

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


#[derive(Clone, Copy, Debug)]
struct Hail {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}


impl Hail {
    fn intersect_2d(a: Hail, b: Hail) -> bool {
        let mut det = b.vx * a.vy - a.vx * b.vy;
        if det == 0 { return false; }
        let sgn;
        if det > 0 {
            sgn = 1;
        } else {
            sgn = -1;
            det = -det;
        }
        let lambda = (b.vy * (a.px - b.px) - b.vx * (a.py - b.py)) * sgn;
        let mu = (a.vy * (a.px - b.px) - a.vx * (a.py - b.py)) * sgn;
        if mu <= 0 || lambda <= 0 { return false; }
        let limits = (200000000000000, 400000000000000);
        return limits.0 * det <= a.px * det + a.vx * lambda
            && a.px * det + a.vx * lambda <= limits.1 * det
            && limits.0 * det <= a.py * det + a.vy * lambda
            && a.py * det + a.vy * lambda <= limits.1 * det;
    }
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut hails = Vec::<Hail>::new();
    for line in reader.lines() {
        hails.push(parse(line?));
    }
    solve(&hails);
    Ok(())
}


// Z3 logic from arthomnix's part2: https://github.com/arthomnix/aoc23/blob/master/src/days/day24.rs
// Code from: https://github.com/tymscar/Advent-Of-Code/blob/master/2023/rust/src/day24/part2.rs
fn solve(hails: &Vec<Hail>) {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in hails {
        let pxn = Int::from_i64(&ctx, hailstone.px);
        let pyn = Int::from_i64(&ctx, hailstone.py);
        let pzn = Int::from_i64(&ctx, hailstone.pz);
        let vxn = Int::from_i64(&ctx, hailstone.vx);
        let vyn = Int::from_i64(&ctx, hailstone.vy);
        let vzn = Int::from_i64(&ctx, hailstone.vz);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    println!("{}", x + y + z);
}


fn parse(line: String) -> Hail {
    let mut it = line.split(" @ ");
    let mut left_it = it.next().unwrap().split(',');
    let mut right_it = it.next().unwrap().split(',');
    Hail { 
        px: left_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        py: left_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        pz: left_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        vx: right_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        vy: right_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
        vz: right_it.next().unwrap().split_ascii_whitespace().nth(0).unwrap().parse().unwrap(),
    }
}