
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Registers([i64; 4]);

struct Sample {
    before: Registers,
    after: Registers,
    instr: Instruction,
}

struct Instruction {
    Op: u8, A: u8, B: u8, C: u8,
}

fn parse(dat: &Vec<&str>) -> Vec<Sample> {
    let mut samples: Vec<Sample> = Vec::new();
    let mut ptr = dat.iter();
    
    while let Some(line) = ptr.next() {
        if line.starts_with("Before") {
        }
    }

    return samples;
}

fn main() 
{
    let dat: Vec<&str> = include_str!("Day16.txt").lines().collect();
    let dat_samples = parse(&dat);
}