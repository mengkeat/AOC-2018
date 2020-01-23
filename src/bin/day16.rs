use std::collections::{HashMap, HashSet};
use std::str::{FromStr};
use std::result;
use std::error::Error;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Clone, Debug, Copy, Hash, Eq, PartialEq)]
enum OpCode { 
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Register([i64; 4]);
// type Register = Vec<i64>;

#[derive(Clone, Debug)]
struct Sample {
    before: Register,
    after: Register,
    instr: Instruction,
}

#[derive(Clone, Debug)]
struct Instruction {
    op: u8, a: i64, b: i64, c: i64,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    
    fn from_str(s: &str) -> Result<Instruction> {
        let v: Vec<i64>  = s.split_whitespace().map(|s| 
                                s.trim().parse::<i64>().unwrap()
                            ).collect();
        if v.len()!=4 { 
            err!("Instruction is more than 4 numbers!") 
        }
        else {
            Ok(Instruction{ op: v[0] as u8, a: v[1], b: v[2], c: v[3]} )
        }
    }
}

fn exe(reg: &Register, op_code: &OpCode, a: &i64, b: &i64, c: &i64) -> Register {
    let mut r = reg.clone();
    let r_a = r.0[*a as usize];
    let r_b = r.0[*b as usize];
    r.0[*c as usize] = match op_code {
            OpCode::Addr  => r_a + r_b,
            OpCode::Addi  => r_a + *b,
            OpCode::Mulr  => r_a * r_b,
            OpCode::Muli  => r_a * *b,
            OpCode::Banr  => r_a & r_b,
            OpCode::Bani  => r_a & *b,
            OpCode::Borr  => r_a | r_b,
            OpCode::Bori  => r_a | *b,
            OpCode::Setr  => r_a,
            OpCode::Seti  => *a,
            OpCode::Gtir  => if *a>r_b {1} else {0},
            OpCode::Gtri  => if r_a>*b {1} else {0},
            OpCode::Gtrr  => if r_a>r_b {1} else {0},
            OpCode::Eqir  => if *a==r_b {1} else {0},
            OpCode::Eqri  => if r_a==*b {1} else {0},
            OpCode::Eqrr  => if r_a==r_b {1} else {0},
        };
    return r;
}

fn parse(dat: &Vec<&str>) -> Result<Vec<Sample>> {
    let mut samples: Vec<Sample> = Vec::new();
    let mut ptr = dat.iter();
    
    while let Some(line) = ptr.next() {
        if line.starts_with("Before") {
            let be: Vec<i64> = line[9..19].split(",").map(|s| s.trim().parse::<i64>().unwrap() ).collect();
            let instr: Instruction = ptr.next().map(|l| l.parse().unwrap() ).unwrap();
            let af: Vec<i64> = ptr.next().unwrap()[9..19]
                                .split(",").map(|s| s.trim().parse::<i64>().unwrap() ).collect();
            samples.push( Sample{ 
                before: Register( [be[0], be[1], be[2], be[3]] ),
                after:  Register( [af[0], af[1], af[2], af[3]] ), 
                instr: instr,
            });
            ptr.next();                                 
        }
        else { // To be filled in later to cater for program
            break;
        }
    }
    return Ok(samples);
}

fn compatible_ocode(s: &Sample) -> HashSet<OpCode> {
    let ocode_lst: Vec<OpCode> = vec![
        OpCode::Addr, OpCode::Addi,
        OpCode::Mulr, OpCode::Muli,
        OpCode::Banr, OpCode::Bani,
        OpCode::Borr, OpCode::Bori,
        OpCode::Setr, OpCode::Seti,
        OpCode::Gtir, OpCode::Gtri, OpCode::Gtrr,
        OpCode::Eqir, OpCode::Eqri, OpCode::Eqrr,
    ];
    ocode_lst.iter()
        .filter_map(|op| 
            if exe(&s.before, op, &s.instr.a, &s.instr.b, &s.instr.c) == s.after {
                Some(*op)
            }
            else { None }
        )
        .collect()
}

fn part1(samples: &Vec<Sample>) {
    let n_samples = samples.iter()
        .map(|s| compatible_ocode(s).len() )
        .filter(|l| *l >= 3 )
        .count();
    println!("Part 1: {}", n_samples);
}


fn decode_opcode(samples: &Vec<Sample>) -> HashMap<u8, OpCode> {
    let op_sample = (0..16).filter_map(|i| samples.iter().find(|s| s.instr.op ==i)).collect::<Vec<_>>();
    let mut cand: HashMap<u8, HashSet<OpCode>> = op_sample.iter().map(|s| (s.instr.op, compatible_ocode(s))).collect();
    let mut op_code_map: HashMap<u8, OpCode> = HashMap::new();

    while op_code_map.len()!=16 {
        let (&n, s) = cand.iter().find(|(_, op_cand)| op_cand.len()==1).unwrap();
        let v = *s.iter().nth(0).unwrap();
        op_code_map.insert(n, v);

        cand.remove(&n);
        for (_, op_set) in cand.iter_mut() {
            op_set.remove(&v);            
        }
    }
    return op_code_map;
}

fn part2(samples: &Vec<Sample>) {
    let op_code_map = decode_opcode(samples);
    println!("{:?}", op_code_map);
}

fn main() {
    let dat: Vec<&str> = include_str!("Day16.txt").lines().collect();
    let mut dat_samples = parse(&dat).unwrap();

    part1(&dat_samples);
    part2(&dat_samples);
}