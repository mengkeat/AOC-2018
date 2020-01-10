
struct Node {
    childs: Vec<Node>,
    meta: Vec<u16>
}

fn make_node(d: &mut Vec<u16>) -> Node 
{
    let n_child = d.pop().unwrap();
    let n_meta = d.pop().unwrap();

    let mut childs = vec![];
    for _ in 0..n_child {
        childs.push(make_node(d));
    }

    let mut meta = vec![];
    for _ in 0..n_meta {
        meta.push(d.pop().unwrap());
    }
    return Node{ childs, meta };
}

fn sum_meta(n: &Node) -> u16
{
    return n.childs.iter().map(sum_meta).sum::<u16>() + n.meta.iter().sum::<u16>();
}

fn value(n: &Node) -> u16
{
    if n.childs.len()==0{
        return n.meta.iter().sum::<u16>();
    }
    else {
        return n.meta.iter()
                .filter_map(|i| n.childs.get((*i as usize)-1) )
                .map(value)
                .sum::<u16>();
    }
}

fn main()
{
    let dat: Vec<u16> = include_str!("Day08.txt").trim()
                        .split(' ')
                        .map(|s| s.parse().unwrap() )
                        .collect();
    let rev_dat: Vec<u16> = dat.into_iter().rev().collect();
    let node = make_node(&mut rev_dat.clone());

    println!("Part 1: {}", sum_meta(&node));
    println!("Part 2: {}", value(&node));
}