use std::collections::VecDeque;

const R: usize = 190221;
// const R: usize = 9;

struct Recipes {
    e1: usize,
    e2: usize,
    seq: Vec<u8>
}

impl Recipes {
    fn new() -> Self {
        let s: Vec<u8> = vec![3, 7];
        return Recipes { e1: 0, e2: 1, seq: s};
    }    
    
    fn next(&mut self) {
        let n = self.seq[self.e1]+self.seq[self.e2];
        if n>=10 {
            self.seq.push(1);
            self.seq.push(n%10);
        }
        else {
            self.seq.push(n);
        }
        self.e1 = (self.e1+self.seq[self.e1] as usize+1) % self.seq.len();
        self.e2 = (self.e2+self.seq[self.e2] as usize+1) % self.seq.len();
    }
}

fn part1(rec: &mut Recipes) 
{
    while rec.seq.len() < R+10 {
        rec.next();
    }
    let score: String = rec.seq[R..].iter().map(|i| i.to_string()).collect();
    println!("Part 1: {}", score);
}

fn part2(rec: &mut Recipes)
{
    let sz = R.to_string().len();
    let mut i: usize = 0;
    let mut a: VecDeque<u8> = VecDeque::new();

    loop {
        a.pop_front();
        while a.len()<sz {
            if i>=rec.seq.len() { rec.next(); }
            a.push_back(rec.seq[i]);
            i += 1;
        }
        let curr: usize = a.iter().fold(0, |sum, &i| sum*10+i as usize);
        if curr == R {
            println!("Part 2: {}", i-sz);
            return;
        }
    }
}

fn main() 
{
    let mut rec = Recipes::new();
    part1(&mut rec);
    part2(&mut rec);
}