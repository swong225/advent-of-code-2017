use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

extern crate regex;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Program {
    weight: u64,
    children: Vec<String>,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+) \((\d+)\)(?: -> )?(.*)?").unwrap();
}

fn parse_line(l : &String) -> (String, Program) {
    let cap = RE.captures(&l).unwrap();
    let base = &cap[1];
    let w: u64 = String::from(&cap[2]).parse().unwrap();
    
    let p : Program;

    if !cap[3].is_empty() {
        let child_str: Vec<&str> = cap[3].split_whitespace().collect();
        let clean_children: Vec<String> = child_str.iter().map(|x| x.replace(",", "")).collect();
        p = Program { weight: w, children: clean_children};
    } else {
        p = Program { weight: w, children: Vec::new()};
    }

    return (base.to_string(), p);
}

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut pmap = HashMap::<String, Program>::new();

    for line in file.lines() {
        let (id, program) = parse_line(&line.unwrap());
        pmap.insert(id, program);
    }

    // Track all child nodes
    let mut kids: HashSet<String> = HashSet::new();

    for (_, program) in &pmap {
        for child in &program.children {
            kids.insert(child.clone());
        }
    }

    // If node is not in the kid set, the it's the root
    let mut root = String::new();
    for (pid, _) in &pmap {
        if !kids.contains(pid) {
            // Root!
            root = pid.clone();
        }
    }

    println!("Root node is {} - {:?}", root, pmap.get(&root).unwrap());

    ascend(&root, &pmap);
}

fn subtower_weight(pid: &String, nodes: &HashMap<String, Program>) -> u64 {
    let p = nodes.get(pid).unwrap();
    let mut weight = p.weight;
    for child in &p.children {
        weight = weight + subtower_weight(child, &nodes);
    }
    return weight;
}

fn ascend(name: &str, nodes: &HashMap<String, Program>) {
    let node = nodes.get(name).unwrap();
    let weights: Vec<u64> = node.children
        .iter()
        .map(|x| subtower_weight(x, &nodes))
        .collect();
    for w in &weights {
        if *w != weights[0] {
            println!("Mismatch {} != {}", w, weights[0]);
            break;
        }
    }
    if node.children.len() > 0 {
        println!(
            "{} ({}) -> {:?}",
            name,
            node.weight,
            node.children.iter().zip(weights).collect::<Vec<_>>()
        );
    }
    for c in &node.children {
        ascend(c, &nodes);
    }
}