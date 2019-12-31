use std::collections::HashMap;

fn part1(dat: &str) -> u32
{
    let mut two: u32 = 0;
    let mut three: u32 = 0;
    for line in dat.lines() {
        let mut freq = HashMap::new();
        line.chars().for_each(|c| *freq.entry(c).or_default() += 1 );
        two += freq.values().any(|&n: &u32| n==2) as u32;
        three += freq.values().any(|&n: &u32| n==3) as u32;
    }
    two*three
}

fn part2(dat: &str) -> Option<String>
{
    for l1 in dat.lines() {
        for l2 in dat.lines() {
            let s: String = l1.chars().zip(l2.chars()) 
                    .filter_map(|(a,b)|  if a == b { Some(a) } else {None})
                    .collect();
            if s.len() == l2.len()-1 {
                return Some(s);
            }
        }
    }
    return None;
}

fn main()
{
    let dat = include_str!("Day02.txt");
    println!("Part 1: {}", part1(dat));
    println!("Part 2: {}", part2(dat).unwrap());
}
