use std::collections::{HashMap};
use std::io::{self, Read};

type ClayMap = HashMap<(i64, i64), u8>;

fn get_map(dat: &str) -> ClayMap {
    let mut m = ClayMap::new();
    for line in dat.lines() {
        let xy: Vec<Vec<&str>> = line.split(", ").map(|s| s.split("=").collect() ).collect();
        let range: Vec<i64> = xy[1][1].split("..").map(|x| x.parse().unwrap() ).collect();
        let fixed_coord = xy[0][1].parse().unwrap();
        let fixed_x = xy[0][0].chars().nth(0)==Some('x');
 
        for r in range[0]..range[1]+1 {
            let x = match fixed_x { true => fixed_coord, _ => r };
            let y = match fixed_x { true => r, _ => fixed_coord };
            m.insert((x,y), 1);
        }
    }
    return m;
}

fn main() {
    // Let's change input to take in files when necessary
    let mut dat = String::new();
    io::stdin().read_to_string(&mut dat);

    let clay_map = get_map(&dat);
}