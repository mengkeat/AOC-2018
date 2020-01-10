
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
 
    for (&n, &c) in dep.iter() {
        if c==0 { cand.push(Reverse(n)); }
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

fn work(t: &u16, curr_work: &mut Vec<(u16, char)>, cand_tasks: &mut BinaryHeap<Reverse<char>>) {
    while curr_work.len()<5 && cand_tasks.len()!=0 {
        let Reverse(task) = cand_tasks.pop().unwrap();
        curr_work.push((t+(task as u16)-4, task));
    }
}

fn part2(g: &GraphType, mut dep: HashMap<char, u8>) -> u16
{
    let mut t: u16 = 0;
    let mut cand_tasks: BinaryHeap<Reverse<char>> = BinaryHeap::new();
    let mut curr_work: Vec<(u16, char)> = Vec::new();

    for (&n, &c) in dep.iter() {
        if c==0 { cand_tasks.push(Reverse(n)); }
    }
    work(&t, &mut curr_work, &mut cand_tasks);

    while curr_work.len()!=0 || cand_tasks.len()!=0 {
        let (min_time, min_task) = curr_work.iter().min_by_key(|(t,_)| t).unwrap().clone();
        t = min_time;
        curr_work.retain(|(tm,c)| *tm!=min_time && *c!=min_task);
        for n in g.get(&min_task).unwrap() {
            let c = dep.entry(*n).or_default();
            *c -= 1;
            if *c==0 {
                cand_tasks.push(Reverse(*n));
            }
        }
        work(&t, &mut curr_work, &mut cand_tasks);
    }

    return t;
}

fn main()
{
    let dat = include_str!("Day07.txt").trim();
    let (graph, dep_count) = make_graph(dat);

    let order = part1(&graph, dep_count.clone());
    println!("Part 1: {}", order);
    println!("Part 2: {}", part2(&graph, dep_count.clone()));
}