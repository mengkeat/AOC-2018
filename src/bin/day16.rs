
use std::str::{FromStr};
use std::result;
use std::error::Error;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<Error>>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Registers([i64; 4]);

#[derive(Clone, Debug)]
struct Sample {
    before: Registers,
    after: Registers,
    instr: Instruction,
}

#[derive(Clone, Debug)]
struct Instruction {
    Op: u8, A: u8, B: u8, C: u8,
}

impl FromStr for Instruction {
    type Err = Box<Error>;
    
    fn from_str(s: &str) -> Result<Instruction> {
        let v: Vec<u8>  = s.split_whitespace().map(|s| 
                                s.trim().parse::<u8>().unwrap()
                            ).collect();
        if v.len()!=4 { 
            err!("Instruction is more than 4 numbers!") 
        }
        else {
            Ok(Instruction{ Op: v[0], A: v[1], B: v[2], C: v[3]} )
        }
    }
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
                before: Registers([be[0], be[1], be[2], be[3]]),
                after:  Registers([af[0], af[1], af[2], af[3]]),
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

fn main() 
{
    let dat: Vec<&str> = include_str!("Day16.txt").lines().collect();
    let dat_samples = parse(&dat).unwrap();

    println!("{:?}", dat_samples.len());
}