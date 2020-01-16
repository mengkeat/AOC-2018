use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

type RuleSet = HashMap<String, char>;
type State = VecDeque<char>;

#[derive(Debug)]
struct TunnelPots {
    pots: State, 
    rules: RuleSet,
    offset: i32
}

impl TunnelPots {
    fn new(inp: &Vec<&str>) -> TunnelPots {
        let mut pots = TunnelPots { pots: State::new(), rules: RuleSet::new(), offset: 0 };
        pots.set_pot(&inp[0][15..]);
        pots.set_rules(&inp[2..]);
        return pots;
    }

    fn normalize_state(&mut self) {
        while self.pots.front()==Some(&'.') { 
            self.pots.pop_front(); 
            self.offset += 1;
        }
        for _ in 0..5 { 
            self.pots.push_front('.'); 
            self.offset -= 1;
        }
        while self.pots.back()==Some(&'.') { self.pots.pop_back(); }
        for _ in 0..5 { self.pots.push_back('.'); }
    }

    fn set_pot(&mut self, input: &str) {
        self.pots.clear();
        self.pots = VecDeque::from_iter(input.chars());
    }

    fn set_rules(&mut self, rule_str: &[&str]) {
        self.rules.clear();
        for rstr in rule_str {
            self.rules.insert(rstr[..5].to_string(), rstr.chars().nth(9).unwrap());
        }
    }
}

fn get_count(s: &State, offset: i32) -> i32 {
    return s.iter()
        .enumerate()
        .filter_map(|(i, &e)| if e=='#' { Some(offset+i as i32) } else {None} )
        .sum();
}

impl Iterator for TunnelPots {
    type Item = (State, i32);
    
    fn next(&mut self) -> Option<(State, i32)> {
        let mut new_state = State::new();
        self.normalize_state();
        for i in 0..self.pots.len()-5 {
            let k: String = (i..i+5).map(|a| self.pots[a] ).collect();
            new_state.push_back(*self.rules.entry(k).or_insert('.'));
        }
        self.pots = new_state;
        self.offset += 2;
        return Some((self.pots.clone(), self.offset));
    }
}

fn main()
{
    let gen: i64 = 50_000_000_000;
    let dat: Vec<&str> = include_str!("Day12.txt").lines().collect();
    let pots = TunnelPots::new(&dat);

    let mut count: u32 = 0;
    let mut prev: i32 = 0;
    for (st, off) in pots.take(520) {
        count += 1;
        let s: String = st.iter().map(|c| c).collect();
        let curr_count = get_count(&st, off);
        println!("Count for {:?} = {}  iteration: {} diff: {}", s, curr_count, count, curr_count-prev);
        prev = curr_count;
    }
    // 78 is derived from the the difference printed previously which stablizes from iteration to iteration
    println!("Part 2: {} ", prev as i64+ (gen-count as i64)*78);
}