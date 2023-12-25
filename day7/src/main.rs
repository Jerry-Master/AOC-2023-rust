use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::collections::HashMap;
use std::cmp::{Eq, PartialEq, Ord, PartialOrd};
use core::cmp::Ordering;

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
#[derive(Eq, PartialEq)]
struct Hand {
    strength: u32, first: char, second: char, third: char, fourth: char, fifth: char,
    bid: u32
}

fn cmp(a: char, b: char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    let map = HashMap::from([
        ('A', 15), ('K', 14), ('Q', 13),
        ('J', 2), ('T', 11), ('9', 10),
        ('8', 9), ('7', 8), ('6', 7),
        ('5', 6), ('4', 5), ('3', 4),
        ('2', 3),
    ]);
    if map[&a] < map[&b] {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength < other.strength {
            return Ordering::Less;
        }
        if self.strength > other.strength {
            return Ordering::Greater;
        }
        if cmp(self.first, other.first) != Ordering::Equal {
            return cmp(self.first, other.first);
        }
        if cmp(self.second, other.second) != Ordering::Equal {
            return cmp(self.second, other.second);
        }
        if cmp(self.third, other.third) != Ordering::Equal {
            return cmp(self.third, other.third);
        }
        if cmp(self.fourth, other.fourth) != Ordering::Equal {
            return cmp(self.fourth, other.fourth);
        }
        if cmp(self.fifth, other.fifth) != Ordering::Equal {
            return cmp(self.fifth, other.fifth);
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Hand {
    fn new(line: &str, bid: u32) -> Self{
        let first: char = line.chars().nth(0).unwrap();
        let second: char = line.chars().nth(1).unwrap();
        let third: char = line.chars().nth(2).unwrap();
        let fourth: char = line.chars().nth(3).unwrap();
        let fifth: char = line.chars().nth(4).unwrap();
        let mut hand  = Hand{
            first, second, third, fourth, fifth, 
            strength: 0, bid
        }; 
        hand.compute_strength();
        return hand;
    }


    fn compute_strength(&mut self) {
        if self.five_equal() {
            self.strength = 6; return;
        }
        if self.four_equal() {
            self.strength = 5; return;
        }
        if self.three_equal() {
            if self.count_pairs() == 2 {
                self.strength = 4; return;
            } else {
                self.strength = 3; return;
            }
        }
        if self.count_pairs() == 2 {
            self.strength = 2; return;
        }
        if self.count_pairs() == 1 {
            self.strength = 1; return;
        }
        self.strength = 0; return;
    }


    fn five_equal(&self) -> bool {
        let mut freqs = HashMap::from(
            [
                ('A', 0), ('K', 0), ('Q', 0),
                ('J', 0), ('T', 0), ('9', 0),
                ('8', 0), ('7', 0), ('6', 0),
                ('5', 0), ('4', 0), ('3', 0),
                ('2', 0),
            ]
        );
        *freqs.entry(self.first).or_insert(0) += 1;
        *freqs.entry(self.second).or_insert(0) += 1;
        *freqs.entry(self.third).or_insert(0) += 1;
        *freqs.entry(self.fourth).or_insert(0) += 1;
        *freqs.entry(self.fifth).or_insert(0) += 1;
        for (k, v) in &freqs {
            if v + freqs[&'J'] == 5 && *k != 'J' {
                return true;
            }
            if *v == 5 {
                return true;
            }
        }
        false
    }


    fn four_equal(&self) -> bool {
        let mut freqs = HashMap::from(
            [
                ('A', 0), ('K', 0), ('Q', 0),
                ('J', 0), ('T', 0), ('9', 0),
                ('8', 0), ('7', 0), ('6', 0),
                ('5', 0), ('4', 0), ('3', 0),
                ('2', 0),
            ]
        );
        *freqs.entry(self.first).or_insert(0) += 1;
        *freqs.entry(self.second).or_insert(0) += 1;
        *freqs.entry(self.third).or_insert(0) += 1;
        *freqs.entry(self.fourth).or_insert(0) += 1;
        *freqs.entry(self.fifth).or_insert(0) += 1;
        for (k, v) in &freqs {
            if v + freqs[&'J'] == 4 && *k != 'J' {
                return true;
            }
            if *v == 4 {
                return true;
            }
        }
        false
    }


    fn three_equal(&self) -> bool {
        let mut freqs = HashMap::from(
            [
                ('A', 0), ('K', 0), ('Q', 0),
                ('J', 0), ('T', 0), ('9', 0),
                ('8', 0), ('7', 0), ('6', 0),
                ('5', 0), ('4', 0), ('3', 0),
                ('2', 0),
            ]
        );
        *freqs.entry(self.first).or_insert(0) += 1;
        *freqs.entry(self.second).or_insert(0) += 1;
        *freqs.entry(self.third).or_insert(0) += 1;
        *freqs.entry(self.fourth).or_insert(0) += 1;
        *freqs.entry(self.fifth).or_insert(0) += 1;
        for (k, v) in &freqs {
            if v + freqs[&'J'] == 3 && *k != 'J' {
                return true;
            }
            if *v == 3 {
                return true;
            }
        }
        false
    }


    fn count_pairs(&self) -> u32 {
        let mut freqs = HashMap::from(
            [
                ('A', 0), ('K', 0), ('Q', 0),
                ('J', 0), ('T', 0), ('9', 0),
                ('8', 0), ('7', 0), ('6', 0),
                ('5', 0), ('4', 0), ('3', 0),
                ('2', 0),
            ]
        );
        *freqs.entry(self.first).or_insert(0) += 1;
        *freqs.entry(self.second).or_insert(0) += 1;
        *freqs.entry(self.third).or_insert(0) += 1;
        *freqs.entry(self.fourth).or_insert(0) += 1;
        *freqs.entry(self.fifth).or_insert(0) += 1;
        let mut pairs = 0;
        for (k, v) in &freqs {
            if *v >= 2 && *k != 'J' {
                pairs += 1;
            }
        }
        if pairs == 0 {
            for (k, v) in &freqs {
                if *v > 0 && *k == 'J' {
                    pairs = 1;
                }
            }
        }
        return pairs;
    }
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut hands = Vec::<Hand>::new();
    for line in reader.lines() {
        let line = line?;
        hands.push(
            Hand::new(
                line.split(' ').nth(0).unwrap(), 
                line.split(' ').nth(1).unwrap().parse::<u32>().unwrap()
            )
        );
    }
    hands.sort_unstable();
    // println!("{:#?}", hands);
    let mut res = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        res += (i as u32 + 1) * hand.bid;
    }
    println!("{}", res);
    Ok(())
}
