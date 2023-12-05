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
            src_start: range_values[1],
            src_end: range_values[1] + range_values[2] - 1,
            dst_start: range_values[0]
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

    let mut seeds = Vec::<u64>::new();
    let mut seed2soil = Map::new();
    let mut soil2fert = Map::new();
    let mut fert2water = Map::new();
    let mut water2light = Map::new();
    let mut light2temp = Map::new();
    let mut temp2hum = Map::new();
    let mut hum2loc = Map::new();
    let mappings = [
        &mut seed2soil, &mut soil2fert, &mut fert2water,
        &mut water2light, &mut light2temp, &mut temp2hum,
        &mut hum2loc
    ];
    let mut idx = 0;
    // Read
    for line in reader.lines() {
        let line = line?;
        if seeds.len() == 0 {
            seeds = line
                    .split(": ")
                    .nth(1).unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
        } else if !line.is_empty() {
            if line.chars().nth(0).unwrap().is_ascii_digit() {
                mappings[idx-1].push(&line);
            }
        } else {
            idx += 1;
        }
    }
    // Sort ranges for binary search
    seed2soil.sort();
    soil2fert.sort();
    fert2water.sort();
    water2light.sort();
    light2temp.sort();
    temp2hum.sort();
    hum2loc.sort();
    // Traverse mappings
    let mut locations = Vec::<u64>::new();
    for seed in seeds {
        let soil = seed2soil.map(seed);
        let fert = soil2fert.map(soil);
        let water = fert2water.map(fert);
        let light = water2light.map(water);
        let temp = light2temp.map(light);
        let hum = temp2hum.map(temp);
        let loc = hum2loc.map(hum);
        locations.push(loc);
    }
    // Find minimum
    println!("{:?}", locations.iter().min().unwrap());
    Ok(())
}
