use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::cmp::{max, min, Ord, Ordering};

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


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instr {
    dir: Dir,
    length: usize,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum CornerType {
    UL,
    UR,
    DL,
    DR
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Corner {
    ctype: CornerType,
    i: usize,
    j: usize
}


impl Ord for Corner {
    fn cmp(&self, other: &Self) -> Ordering {
        self.i.cmp(&other.i).then_with(|| self.j.cmp(&other.j))
    }
}


impl PartialOrd for Corner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut instructs = Vec::<Instr>::new();
    for line in reader.lines() {
        let line = line?;
        let mut words = line.split_whitespace();
        let (hex_dir, hex_length) = decode_hex(words.nth(2).unwrap());
        instructs.push(
            Instr {
                dir: match hex_dir {
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    'U' => Dir::Up,
                    'D' => Dir::Down,
                    _ => panic!(),
                },
                length: hex_length,
            }
        )
    }
    let start = get_shape(&instructs);  // Board of m x n
    let mut nodes = Vec::<Corner>::new();
    read_nodes(&mut nodes, &instructs, start);
    nodes.sort();
    let mut buckets: Vec<Vec<Corner>> = vec![];
    fill_buckets(&mut buckets, &nodes);
    let res = get_area(&buckets);
    println!("{}", res);
    Ok(())
}


fn decode_hex(hex: &str) -> (char, usize) {
    let length = usize::from_str_radix(&hex[2..3], 16).unwrap() * 65536
        + usize::from_str_radix(&hex[3..5], 16).unwrap() * 256
        + usize::from_str_radix(&hex[5..7], 16).unwrap();
    let dir = match hex.chars().nth(7).unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!(),
        };
    (dir, length)
}


fn get_area(buckets: &Vec<Vec<Corner>>) -> usize {
    let mut last_bucket = buckets[0].clone();
    let mut total = sum_interval_lengths(&last_bucket);
    for k in 1..buckets.len() {
        let bucket = &buckets[k];
        total += get_area_from_buckets(bucket, &last_bucket);
        update_bucket(&mut last_bucket, bucket);
    }
    total
}


fn update_bucket(last_bucket: &mut Vec<Corner>, bucket: &Vec<Corner>) {
    for element in last_bucket.iter_mut() {
        element.i = bucket[0].i;
    }
    last_bucket.append(&mut bucket.clone());
    last_bucket.sort();
    // Remove duplicates
    let mut aux_bucket = Vec::<Corner>::new();
    for k in 0..last_bucket.len() {
        if k > 0 && k < last_bucket.len() - 1 && last_bucket[k].j != last_bucket[k-1].j && last_bucket[k].j != last_bucket[k+1].j {
            aux_bucket.push(last_bucket[k]);
        } else if k == 0 && last_bucket[k].j != last_bucket[k+1].j {
            aux_bucket.push(last_bucket[k]);
        } else if k == last_bucket.len() - 1 && last_bucket[k].j != last_bucket[k-1].j {
            aux_bucket.push(last_bucket[k]);
        }
    }
    *last_bucket = aux_bucket.clone();
}


fn get_area_from_buckets(b1: &Vec<Corner>, b2: &Vec<Corner>) -> usize {
    let dist = b1[0].i - b2[0].i;
    let length = sum_interval_lengths(b2);
    let extra = sum_extra_length(b1, b2);
    dist * length + extra
}


fn sum_extra_length(b1: &Vec<Corner>, b2: &Vec<Corner>) -> usize {
    assert!(b1.len() % 2 == 0);
    let mut total = 0;
    let mut is_in = false;
    let mut k2 = 0;
    for k in 0..b1.len() / 2 {
        while k2 < b2.len() && b2[k2].j < b1[2 * k + 1].j {
            if b2[k2].j != b1[2 * k].j {
                is_in = !is_in;
            }
            k2 += 1;
        }
        if k2 < b2.len() && b2[k2].j == b1[2 * k + 1].j { k2 += 1; }
        if (b1[2 * k].ctype == CornerType::DR && b1[2 * k + 1].ctype == CornerType::UL && !is_in) ||
         (b1[2 * k].ctype == CornerType::UR && b1[2 * k + 1].ctype == CornerType::DL && is_in) {
            total += b1[2 * k + 1].j - b1[2 * k].j;
        }
        if b1[2 * k].ctype == CornerType::DR && b1[2 * k + 1].ctype == CornerType::DL && !is_in {
            total += b1[2 * k + 1].j - b1[2 * k].j + 1;
        }
        if b1[2 * k].ctype == CornerType::UR && b1[2 * k + 1].ctype == CornerType::UL && is_in {
            total += b1[2 * k + 1].j - b1[2 * k].j - 1;
        }
        if (b1[2 * k].ctype == CornerType::DR && b1[2 * k + 1].ctype == CornerType::UL) ||
         (b1[2 * k].ctype == CornerType::UR && b1[2 * k + 1].ctype == CornerType::DL) {
            is_in = !is_in;
        }
    }
    total
}


fn sum_interval_lengths(b: &Vec<Corner>) -> usize {  // Intervals come in pairs of nodes
    assert!(b.len() % 2 == 0);
    let mut total = 0;
    for k in 0..b.len() / 2 {
        total += b[2 * k + 1].j - b[2 * k].j + 1;
    }
    total
}


fn fill_buckets(buckets: &mut Vec<Vec<Corner>>, nodes: &Vec<Corner>) {
    buckets.push(vec![nodes[0]; 1]);
    let mut prev_i = nodes[0].i;
    for k in 1..nodes.len() {
        let node = nodes[k];
        if prev_i != node.i {
            buckets.push(vec![node; 1]);
        } else {
            let n = buckets.len();
            buckets[n - 1].push(node);
        }
        prev_i = node.i;
    }
}


fn read_nodes(nodes: &mut Vec<Corner>, instructs: &Vec<Instr>, start: (usize, usize)) {
    let (mut i, mut j) = start;
    let mut last_dir = instructs[0].dir;
    let first_dir = instructs[0].dir;
    match instructs[0].dir {
        Dir::Left => j -= instructs[0].length,
        Dir::Right => j += instructs[0].length,
        Dir::Down => i += instructs[0].length,
        Dir::Up => i -= instructs[0].length,
    };
    for k in 1..instructs.len() {
        let instr = instructs[k];
        nodes.push(Corner { ctype: match (last_dir, instr.dir) {
            (Dir::Left, Dir::Up) => CornerType::UR,
            (Dir::Left, Dir::Down) => CornerType::DR,
            (Dir::Right, Dir::Up) => CornerType::UL,
            (Dir::Right, Dir::Down) => CornerType::DL,
            (Dir::Up, Dir::Left) => CornerType::DL,
            (Dir::Down, Dir::Left) => CornerType::UL,
            (Dir::Up, Dir::Right) => CornerType::DR,
            (Dir::Down, Dir::Right) => CornerType::UR,
            (_, _) => panic!(),
        }, i: i, j: j });
        last_dir = instr.dir;
        match instr.dir {
            Dir::Left => j -= instr.length,
            Dir::Right => j += instr.length,
            Dir::Down => i += instr.length,
            Dir::Up => i -= instr.length,
        };
    }
    if first_dir != last_dir {
        nodes.push(Corner { ctype: match (last_dir, first_dir) {
            (Dir::Left, Dir::Up) => CornerType::UR,
            (Dir::Left, Dir::Down) => CornerType::DR,
            (Dir::Right, Dir::Up) => CornerType::UL,
            (Dir::Right, Dir::Down) => CornerType::DL,
            (Dir::Up, Dir::Left) => CornerType::DL,
            (Dir::Down, Dir::Left) => CornerType::UL,
            (Dir::Up, Dir::Right) => CornerType::DR,
            (Dir::Down, Dir::Right) => CornerType::UR,
            (_, _) => panic!(),
        }, i: i, j: j });
    }
}


fn get_shape(instructs: &Vec<Instr>) -> (usize, usize) {
    let (mut m, mut n) = (1, 1);
    let (mut m_max, mut n_max) = (1, 1);
    let (mut m_min, mut n_min) = (1, 1);
    for instr in instructs {
        match instr.dir {
            Dir::Left => { n -= instr.length as i32; n_min = min(n, n_min) },
            Dir::Right => { n += instr.length as i32; n_max = max(n, n_max) },
            Dir::Down => { m += instr.length as i32; m_max = max(m, m_max) },
            Dir::Up => { m -= instr.length as i32; m_min = min(m, m_min) },
        }
    }
    ((1 - m_min) as usize, (1 - n_min) as usize)
}
