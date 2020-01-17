
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

        // for i in 0..self.seq.len() {
        //     if i==self.e1 {
        //         print!("({}) ", self.seq[i]);
        //     }
        //     else if i==self.e2 {
        //         print!("[{}] ", self.seq[i]);
        //     }
        //     else {
        //         print!(" {}  ", self.seq[i]);
        //     }
        // }
        // println!();
    }
}

fn main() 
{
    let mut rec = Recipes::new();
    while rec.seq.len() < R+10 {
        rec.next();
    }
    let score: String = rec.seq[R..].iter().map(|i| i.to_string()).collect();
    println!("Part 1: {}", score);
}