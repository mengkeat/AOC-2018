use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

type RuleSet = HashMap<String, char>;
type State = VecDeque<char>;

#[derive(Debug)]
struct TunnelPots {
    pots: State, 
    rules: RuleSet
}

impl TunnelPots {
    fn new(inp: &Vec<&str>) -> TunnelPots {
        let mut pots = TunnelPots { pots: State::new(), rules: RuleSet::new() };
        pots.set_pot(&inp[0][15..]);
        pots.set_rules(&inp[2..]);
        return pots;
    }

    fn normalize_state(&mut self) {
        while self.pots.front()==Some(&'.') { self.pots.pop_front(); }
        for _ in 0..5 { self.pots.push_front('.'); }
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

impl Iterator for TunnelPots {
    type Item = State;
    
    fn next(&mut self) -> Option<State> {
        let new_state = State::new();
        self.normalize_state();

        return Some(new_state);
    }
}

fn main()
{
    let dat: Vec<&str> = include_str!("Day12.txt").lines().collect();
    let mut pots = TunnelPots::new(&dat);
    println!("{:?}", pots);
}