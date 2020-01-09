
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

type GraphType = HashMap<char, Vec<char>>;

fn make_graph(s: &str) ->  (GraphType, HashMap<char, u8>)
{
    let mut graph = GraphType::new();
    let mut dep_count: HashMap<char, u8> = HashMap::new();

    s.lines().for_each(|l| {
        let src  = l.chars().nth(5).unwrap();
        let dest = l.chars().nth(36).unwrap();
        graph.entry(src).or_default().push(dest);
        graph.entry(dest).or_default();

        *dep_count.entry(dest).or_default() += 1;
        dep_count.entry(src).or_default();
    });

    return (graph, dep_count);
}

fn part1(g: &GraphType, mut dep: HashMap<char, u8>) -> String
{
    let mut order = String::new();
    let mut cand: BinaryHeap<Reverse<char>> = BinaryHeap::new();
 
    for (&n, c) in dep.iter() {
        if *c==0 { cand.push(Reverse(n)); }
    }

    while cand.len()!=0 {
        let Reverse(curr_node) = cand.pop().unwrap();
        order.push(curr_node);

        for n in g.get(&curr_node).unwrap() {
            let c = dep.entry(*n).or_default();
            *c -= 1;
            if *c==0 {
                cand.push(Reverse(*n));
            }
        }
    }

   return order;
}

fn part2(s: &String) -> u16
{
    let mut t: u16 = 0;
    let mut tasks: Vec<u16> = s.chars().rev().map(|c| (c as u16)-4).collect();
    let mut workers = [0u16; 5];

    loop {
        if tasks.len()>0 {
            match workers.iter().find(|&&a| a==0) {
                Some(i) => workers[*i as usize] = tasks.pop().unwrap(),
                None => {
                    let m: u16 = *workers.iter().min().unwrap();
                    for w in workers.iter_mut() {
                        *w -= m;
                    }
                    t += m;
                }
            }
        }
        else {
            return t+ workers.iter().max().unwrap();
        }
    }
}

fn main()
{
    let dat = include_str!("Day07.txt").trim();
    let (graph, dep_count) = make_graph(dat);

    let order = part1(&graph, dep_count.clone());
    println!("Part 1: {}", order);
    println!("Part 2: {}", part2(&order));
}