
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

type GraphType = HashMap<char, HashSet<char>>;

fn make_graph(s: &str) ->  GraphType
{
    let mut graph = GraphType::new();
    let mut src_set = HashSet::new();

    s.lines().for_each(|l| {
        let src  = l.chars().nth(5).unwrap();
        let dest = l.chars().nth(36).unwrap();
        graph.entry(dest).or_default().insert(src);
        src_set.insert(src);
    });
    for s in src_set { graph.entry(s).or_default(); }
    return graph;
}

fn part1(mut g: GraphType) -> String
{
    let mut order = String::new();
    let mut cand: BinaryHeap<Reverse<char>> = BinaryHeap::new();
    let start: Vec<char> = g.iter()
        .filter_map(|(n, set)| 
            if set.len()==0 { return Some(*n); } else { return None; }
        ).collect();
    for n in start {
        cand.push(Reverse(n));
    }
    return order;
}

fn main()
{
    let dat = include_str!("Day07.txt").trim();
    let graph = make_graph(dat);

    println!("{:?}", part1(graph.clone()));
}