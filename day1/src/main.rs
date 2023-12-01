use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error, ErrorKind};
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

use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
    word: Option<String>,
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::default() }
    }

    fn insert(&mut self, word: &'static str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::default);
        }
        current.is_end_of_word = true;
        current.word = Some(String::from(word));
    }
}


fn main() -> io::Result<()>{
    let args = Args::parse();
    let file = File::open(args.input_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        sum += calib_value(&line?).ok_or_else(|| 0).map_err(|_err|Error::new(ErrorKind::Other,"Not digit."))?;
    }
    println!("{}", sum);
    Ok(())
}


fn calib_value(line: &str) -> Option<u32> {
    let mut first_digit: u32 = 10;
    let mut last_digit: u32 = 10;
    let dict: HashMap<String, u32> = HashMap::from([
        (String::from("one"), 1), (String::from("two"), 2), 
        (String::from("three"), 3), (String::from("four"), 4), 
        (String::from("five"), 5), (String::from("seight"), 8),
        (String::from("six"), 6), (String::from("seven"), 7), 
        (String::from("eight"), 8), (String::from("nine"), 9),
        (String::from("onine"), 9), (String::from("ninine"), 9),
        (String::from("fone"), 1)
    ]);
    let mut trie = Trie::new();
    trie.insert("one");
    trie.insert("two");
    trie.insert("three");
    trie.insert("four");
    trie.insert("five");
    trie.insert("six");
    trie.insert("seven");
    trie.insert("eight");
    trie.insert("nine");
    trie.insert("seight");
    trie.insert("onine");
    trie.insert("fone");
    trie.insert("ninine");
    let mut current = &trie.root;
    for char in line.chars() {
        if char.is_digit(10) {
            if first_digit == 10 {
                first_digit = char.to_digit(10)?;
            }
            last_digit = char.to_digit(10)?;
            current = &trie.root;
        } else {
            if let Some(next_node) = current.children.get(&char) {
                current = next_node;
                if next_node.is_end_of_word {
                    // dbg!("{}", current);
                    // println!("{}", current.word.as_ref().unwrap());
                    if first_digit == 10 {
                        first_digit = dict[current.word.as_ref().unwrap()];
                    }
                    last_digit = dict[current.word.as_ref().unwrap()];
                    current = &trie.root; // Reset to root after finding a word
                    // Start new trie with this char
                    if let Some(next_node) = current.children.get(&char) {
                        current = next_node;
                    }
                }
            } else {
                // println!("{}", char);
                // Character not found, reset to the root or to prev char
                current = &trie.root;
                // if let Some(prev_node) = current.children.get(&prev_char) {
                //     if let Some(next_node) = prev_node.children.get(&char) {
                //         current = next_node;
                //     } else {
                //         current = &trie.root;
                //     }
                // } else {
                //     current = &trie.root;
                // }
                // Start new trie with this char
                if let Some(next_node) = current.children.get(&char) {
                    current = next_node;
                }
            }
        }
    }
    // println!("{}, {}", first_digit, last_digit);
    return Some(first_digit * 10 + last_digit);
}
