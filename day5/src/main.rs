use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::cmp::{Eq, PartialEq, Ord, PartialOrd};

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

#[derive(Eq, PartialEq, Ord, PartialOrd, Default)]
#[derive(Debug)]
#[derive(Clone, Copy)]
struct Range {
    src_start: u64,
    src_end: u64,
    dst_start: u64
}

impl Range {
    fn has_in(&self, val: u64) -> bool {
        return self.src_start <= val && val <= self.src_end;
    }

    fn has_left(&self, val: u64) -> bool {
        return self.src_start > val;
    }

    fn has_right(&self, val: u64) -> bool {
        return val > self.src_end;
    }

    fn map(&self, val: u64) -> u64 {
        return self.dst_start + (val - self.src_start);
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>
}


impl Map {
    fn new() -> Self {
        return Map { ranges: Vec::<Range>::new() };
    }

    fn _push(&mut self, val: Range) {
        self.ranges.push(val);
    }

    fn push(&mut self, val: &str) {
        let range_values: Vec<u64> = val
                            .split_whitespace()
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect();
        self._push(Range { 
            dst_start: range_values[1],
            src_end: range_values[0] + range_values[2] - 1,
            src_start: range_values[0]
        });
    }

    fn sort(&mut self) {
        self.ranges.sort_unstable();
    }

    fn len(&self) -> usize {
        return self.ranges.len()
    }

    fn find_interval(&self, val: u64) -> Range {
        let mut l = 0;
        let mut r = self.len();
        while r > l {
            let m = (l + r) / 2;
            if self.ranges[m].has_in(val) {
                return self.ranges[m];
            } else if self.ranges[m].has_left(val) {
                r = m;
            } else if self.ranges[m].has_right(val) {
                l = m+1;
            } else {
                panic!();
            }
        }
        return Range{
            src_start: val,
            src_end: val,
            dst_start: val
        };
    }

    fn map(&self, val: u64) -> u64 {
        return self.find_interval(val).map(val);
    }
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut seeds = Map::new();
    let mut soil2seed = Map::new();
    let mut fert2soil = Map::new();
    let mut water2fert = Map::new();
    let mut light2water = Map::new();
    let mut temp2light = Map::new();
    let mut hum2temp = Map::new();
    let mut loc2hum = Map::new();
    let mappings = [
        &mut soil2seed, &mut fert2soil, &mut water2fert,
        &mut light2water, &mut temp2light, &mut hum2temp,
        &mut loc2hum
    ];
    let mut idx = 0;
    // Read
    for line in reader.lines() {
        let line = line?;
        if seeds.len() == 0 {
            let seed_ranges: Vec<u64> = line
                    .split(": ")
                    .nth(1).unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
            for i in 0..seed_ranges.len()/2 {
                seeds._push(Range{
                    src_start: seed_ranges[2*i+0],
                    src_end: seed_ranges[2*i+0] + seed_ranges[2*i+1] - 1,
                    dst_start: 0
                });
            }
        } else if !line.is_empty() {
            if line.chars().nth(0).unwrap().is_ascii_digit() {
                mappings[idx-1].push(&line);
            }
        } else {
            idx += 1;
        }
    }
    // Sort ranges for binary search
    seeds.sort();
    soil2seed.sort();
    fert2soil.sort();
    water2fert.sort();
    light2water.sort();
    temp2light.sort();
    hum2temp.sort();
    loc2hum.sort();
    // Traverse mappings
    // println!("{:?}", seeds);
    let max = 10000000000;
    for loc in 1..max {
        let hum = loc2hum.map(loc);
        // println!("{}, {}", loc, hum);
        let temp = hum2temp.map(hum);
        // println!("{}, {}", hum, temp);
        let light = temp2light.map(temp);
        // println!("{}, {}", temp, light);
        let water = light2water.map(light);
        // println!("{}, {}", light, water);
        let fert = water2fert.map(water);
        // println!("{}, {}", water, fert);
        let soil = fert2soil.map(fert);
        // println!("{}, {}", fert, soil);
        let seed = soil2seed.map(soil);
        // println!("{}, {}", soil, seed);
        let seed_range = seeds.find_interval(seed);
        // println!("seed {}", seed);
        if seed_range.src_start != seed_range.src_end {
            println!("{}", loc);
            break;
        }
    }
    // Find minimum
    // println!("{:?}", seed2soil);
    // println!("{:?}", soil2fert);
    // println!("{:?}", fert2water);
    // println!("{:?}", water2light);
    // println!("{:?}", light2temp);
    // println!("{:?}", temp2hum);
    // println!("{:?}", hum2loc);
    Ok(())
}
