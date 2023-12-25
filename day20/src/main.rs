use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use clap::Parser;
use std::collections::{HashMap, VecDeque};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone, Copy, Debug)]
struct FlipFlop {
    is_on: bool,
}

impl Default for FlipFlop {
    fn default() -> Self {
        FlipFlop {is_on: false}
    }
}
impl FlipFlop {
    fn propagate(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.is_on = !self.is_on;
                if self.is_on {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Conjunction {
    memory: HashMap<String, Pulse>,
    num_high: usize,
}

impl Conjunction {
    fn new(memory: HashMap<String, Pulse>) -> Self {
        Conjunction { memory: memory, num_high: 0 }
    }

    fn propagate(&mut self, pulse: Pulse, input_id: String) -> Pulse {
        if pulse == Pulse::High && self.memory[&input_id] == Pulse::Low {
            self.num_high += 1;
        } else if pulse == Pulse::Low && self.memory[&input_id] == Pulse::High {
            self.num_high -= 1;
        }
        self.memory.entry(input_id).and_modify(|x| *x = pulse);
        if pulse == Pulse::High && self.num_high == self.memory.len() {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

#[derive(Debug)]
enum NodeType {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster,
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut tmp_graph = HashMap::<String, Vec<String>>::new();
    for line in reader.lines() {
        let line = line?;
        let mut it = line.split(" -> ");
        let left = it.next().unwrap();
        let right = it.next().unwrap();
        tmp_graph.entry(String::from(left)).or_default().append(
            &mut right.split(", ").map(|x| String::from(x)).collect()
        )
    }
    // println!("{:#?}", tmp_graph);
    let mut graph = HashMap::<String, (NodeType, Vec<String>)>::new();
    construct_graph(tmp_graph, &mut graph);
    let (mut nd_count, mut pc_count, mut vd_count, mut tx_count) = (0, 0, 0, 0);
    let (mut nd_first, mut pc_first, mut vd_first, mut tx_first) = (true, true, true, true);
    let update = |x: &mut usize, y: &mut bool, c: usize| {if *x > 0 && *y {
        *y = false;
        *x = c - *x;
    } else if *y {
        *x = c - *x;
    }};
    for count in 1..10000 {
        let (nd_activated, pc_activated, vd_activated, tx_activated) = simulate(&mut graph);
        if nd_activated {
            update(&mut nd_count, &mut nd_first, count);
        }
        if pc_activated {
            update(&mut pc_count, &mut pc_first, count);
        }
        if vd_activated {
            update(&mut vd_count, &mut vd_first, count);
        }
        if tx_activated {
            update(&mut tx_count, &mut tx_first, count);
        }
    }
    println!("nd count: {}", nd_count);
    println!("pc count: {}", pc_count);
    println!("vd count: {}", vd_count);
    println!("tx count: {}", tx_count);
    println!("rx count: {}", mcm(&vec![nd_count, pc_count, vd_count, tx_count]));
    Ok(())
}


fn simulate(graph: &mut HashMap<String, (NodeType, Vec<String>)>) -> (bool, bool, bool, bool) {
    let mut queue = VecDeque::<(String, Pulse, String)>::new();  // left sends pulse to right
    queue.push_back((String::from(""), Pulse::Low, String::from("broadcaster")));
    let (mut nd_activated, mut pc_activated, mut vd_activated, mut tx_activated) = (false, false, false, false); 
    while let Some(transmission) = queue.pop_front() {
        if transmission.1 == Pulse::High && transmission.0 == "nd" { nd_activated = true; }
        if transmission.1 == Pulse::High && transmission.0 == "pc" { pc_activated = true; }
        if transmission.1 == Pulse::High && transmission.0 == "vd" { vd_activated = true; }
        if transmission.1 == Pulse::High && transmission.0 == "tx" { tx_activated = true; }
        let new_pulse_opt = match &mut graph.entry(transmission.2.clone()).or_insert((NodeType::Broadcaster, vec![])).0 {
            NodeType::FlipFlop(f) => f.propagate(transmission.1),
            NodeType::Conjunction(f) => Some(f.propagate(transmission.1, transmission.0)),
            NodeType::Broadcaster => Some(transmission.1),
        };
        if let Some(new_pulse) = new_pulse_opt {
            for child in graph[&transmission.2].1.iter() {
                queue.push_back((transmission.2.clone(), new_pulse, child.clone()));
            }
        }
    }
    (nd_activated, pc_activated, vd_activated, tx_activated)
}


fn construct_graph(tmp_graph: HashMap<String, Vec<String>>, graph: &mut HashMap<String, (NodeType, Vec<String>)>) {
    let mut parents = get_parents(&tmp_graph);
    for (k, v) in tmp_graph {
        let (key, node) = match k.chars().nth(0).unwrap() {
            'b' => (k, NodeType::Broadcaster),
            '%' => (String::from(&k[1..]), NodeType::FlipFlop(FlipFlop::default())),
            '&' => (String::from(&k[1..]), NodeType::Conjunction(Conjunction::new(parents.remove(&k[1..]).unwrap()))),
            _ => panic!(),
        };
        graph.entry(key).or_insert((node, v));
    }
}


fn get_parents(tmp_graph: &HashMap<String, Vec<String>>) -> HashMap<String, HashMap<String, Pulse>> {
    let mut res = HashMap::<String, HashMap<String, Pulse>>::new();
    for (k, v) in tmp_graph {
        let key = match k.chars().nth(0).unwrap() {
            'b' => k.clone(),
            '%' => String::from(&k[1..]),
            '&' => String::from(&k[1..]),
            _ => panic!(),
        };
        for child in v {
            res.entry(child.clone()).or_default().entry(key.clone()).or_insert(Pulse::Low);
        }
    }
    res
}

fn mcm(vec: &Vec<usize>) -> usize {
    let mut mcm = vec[0];
    for i in 1..vec.len() {
        mcm = mcm_binary(mcm, vec[i]);
    }
    return mcm;
}

fn mcm_binary(a: usize, b: usize) -> usize {
    return a * b / mcd_binary(a, b);
}

fn mcd_binary(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return mcd_binary(b, a % b);
}