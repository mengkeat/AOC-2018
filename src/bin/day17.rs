use std::collections::HashMap;
use std::io::{self, Read};
use std::error::Error;
use std::result;

// macro_rules! err {
//     ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
// }

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Clay, Still, Flow
}

type GroundMap = HashMap<(i64, i64), Tile>;

#[derive(Debug, Clone)]
struct Ground {
    map: GroundMap,
    min_y: i64, 
    max_y: i64,
    min_x: i64,
    max_x: i64,
}

impl Ground {

    fn new(dat: &str) -> Ground {
        let mut m = GroundMap::new();
        for line in dat.lines() {
            let xy: Vec<Vec<&str>> = line.split(", ").map(|s| s.split("=").collect() ).collect();
            let range: Vec<i64> = xy[1][1].split("..").map(|x| x.parse().unwrap() ).collect();
            let fixed_coord = xy[0][1].parse().unwrap();
            let fixed_x = xy[0][0].chars().nth(0)==Some('x');
    
            for r in range[0]..range[1]+1 {
                let x = match fixed_x { true => fixed_coord, _ => r };
                let y = match fixed_x { true => r, _ => fixed_coord };
                m.insert((x,y), Tile::Clay);
            }
        }
        let min_x = m.keys().min_by_key(|c| c.0).unwrap().0;
        let max_x = m.keys().max_by_key(|c| c.0).unwrap().0;

        let min_y = m.keys().min_by_key(|c| c.1).unwrap().1;
        let max_y = m.keys().max_by_key(|c| c.1).unwrap().1;
        return Ground{ map: m, min_y: min_y, max_y: max_y, min_x: min_x, max_x: max_x };
    }

    fn print_map(&self) {
        for y in self.min_y..self.max_y+1 {
            for x in self.min_x..self.max_x+1 {
                if self.map.contains_key(&(x,y)) {
                    match self.map[&(x,y)] {
                        Tile::Clay => print!("#"),
                        Tile::Still => print!("~"),
                        Tile::Flow => print!("|"),
                    }
                }
                else { print!(" "); }
            }
            println!();
        }
    }

    // fn start_flow(&mut self) {

    // }

}

fn main() ->  Result<()> {
    // Let's change input to take in files when necessary
    let mut dat = String::new();
    io::stdin().read_to_string(&mut dat)?;

    let ground = Ground::new(&dat);
    ground.print_map();

    Ok(())
}