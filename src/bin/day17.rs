use std::collections::HashMap;
use std::io::{self, Read};
use std::error::Error;
use std::result;

// macro_rules! err {
//     ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
// }

type Result<T> = result::Result<T, Box<dyn Error>>;
type Coord = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Clay, Still, Flow
}

type GroundMap = HashMap<Coord, Tile>;

// fn left(c: &Coord)  -> Coord { (c.0-1, c.1) }
// fn right(c: &Coord) -> Coord { (c.0+1, c.1) }
fn down(c: &Coord)  -> Coord { (c.0, c.1+1) }
fn up(c: &Coord)    -> Coord { (c.0, c.1-1) }

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
        let min_x = self.map.keys().min_by_key(|c| c.0).unwrap().0;
        let max_x = self.map.keys().max_by_key(|c| c.0).unwrap().0;

        let min_y = self.map.keys().min_by_key(|c| c.1).unwrap().1;
        let max_y = self.map.keys().max_by_key(|c| c.1).unwrap().1;
        for y in min_y..max_y+1 {
            for x in min_x..max_x+1 {
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

    // Flows water in the x direction with delta dx.
    // Returns: Stack of explored directions
    //         True if barrier reached. False if flow down
    fn flow_horizontal(&mut self, start: &Coord, dx: i64) -> (Vec<Coord>, bool) {
        let mut barrier = true;
        let mut pts: Vec<Coord> = Vec::new();
        let mut c: Coord = *start;
        while *self.map.entry((c.0+dx, c.1)).or_insert(Tile::Flow) != Tile::Clay {
            c = (c.0+dx, c.1);
            pts.push(c);
            if !self.map.contains_key(&down(&c)) || self.map[&down(&c)]==Tile::Flow {
                barrier = false;
                break;
            }
        }
        (pts, barrier)
    }

    // Flows down from pos and starts filling up. Returns the left and 
    // right points of overflow positions which will flow down if any.
    fn exit_points(&mut self, pos: &Coord) -> (Option<Coord>, Option<Coord>) {
        let mut left_exit: Option<Coord> = None;
        let mut right_exit: Option<Coord> = None;
        let mut c: Coord = *pos;
        while !self.map.contains_key(&down(&c)) {
            c = down(&c);
            if c.1 > self.max_y {
                return (None, None);
            }
            self.map.insert(c, Tile::Flow);
        }
        if self.map[&down(&c)]==Tile::Flow { return (None, None); }
        while left_exit==None && right_exit==None {
            let (left_st, left_barrier)  = self.flow_horizontal(&c, -1);
            let (right_st, right_barrier) = self.flow_horizontal(&c, 1);
            if left_barrier && right_barrier { // barriers on both sides
                for leftc in left_st { self.map.entry(leftc).and_modify(|e| *e = Tile::Still); }
                *self.map.entry(c).or_insert(Tile::Still) = Tile::Still;
                for rightc in right_st {self.map.entry(rightc).and_modify(|e| *e = Tile::Still); }
                c = up(&c);
            }
            else { // one side or both will flow down
                if !left_barrier { left_exit = left_st.last().map(|v| *v); }
                if !right_barrier { right_exit = right_st.last().map(|v| *v); }
                self.map.entry(c).or_insert(Tile::Flow);
            }
        }
        (left_exit, right_exit)
    }

    fn start_flow(&mut self, start: &Coord) {
        let mut st: Vec<Coord> = Vec::new();
        st.push(*start);

        while st.len()!=0 {
            let c = st.pop().unwrap();
            let (let_exit, right_exit) = self.exit_points(&c);
            if let Some(lcoord) = let_exit {
                st.push(lcoord);
            }
            if let Some(rcoord) = right_exit {
                st.push(rcoord);
            }
        }
    }

    fn water_reached(&self) -> usize {
        self.map.iter()
            .filter(|(c, v)| 
                c.1 >= self.min_y && c.1 <=self.max_y &&
                (**v==Tile::Flow || **v==Tile::Still) )
            .count()
    }

    fn water_retained(&self) -> usize {
        self.map.values()
            .filter(|v| **v==Tile::Still )
            .count()
    }
}

fn main() ->  Result<()> {
    // Let's change input to take in files when necessary
    let mut dat = String::new();
    io::stdin().read_to_string(&mut dat)?;

    let mut ground = Ground::new(&dat);
    ground.start_flow(&(500,0));
    ground.print_map();
    println!("Part 1: {}", ground.water_reached());
    println!("Part 2: {}", ground.water_retained());
    Ok(())
}