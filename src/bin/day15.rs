use std::collections::{BTreeMap, HashMap, VecDeque};
use std::error::Error;
use std::result;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum Race { Elf, Goblin }

#[derive(Debug)]
struct Unit {
    race: Race,
    x: isize,
    y: isize,
    attack: i16,
    hp: i16,
    killed: bool,
}

type MapType = BTreeMap<(isize, isize), u16>;

#[derive(Debug)]
struct Cave {
    units: Vec<Unit>,
    map: MapType,
    dist_map_cache: HashMap<(isize, isize), MapType>,   
}

impl Cave {

    fn new(dat: &str) -> Result<Self> {
        let mut cave = Cave {units: Vec::new(), map: MapType::new(), dist_map_cache: HashMap::new() };
        for (y, row_str) in dat.lines().enumerate() {
            for (x, ele) in row_str.char_indices() {
                let _x = x as isize;
                let _y = y as isize;
                let map_ele = match ele {
                    'E' => { cave.units.push( Unit{race: Race::Elf, x: _x, y: _y, attack: 3, hp: 200, killed: false}); 0 },
                    'G' => { cave.units.push( Unit{race: Race::Goblin, x: _x, y: _y, attack: 3, hp: 200, killed: false}); 0 },
                    '.' => 0,
                    '#' => 1,
                    _ => { return err!("Error reading map in from data file!"); }
                };
                cave.map.insert((_y, _x), map_ele);
            }
        }
        cave.compute_all_dist()?;
        Ok(cave)
    }

    fn compute_all_dist(&mut self) -> Result<()> {
        for (&(y,x), &c) in self.map.iter() {
            if c==0 {
                self.dist_map_cache.insert((y,x), self.distance_map((y,x)));
            }            
        }
        Ok(())
    }

    fn distance_map(&self, start: (isize, isize)) -> MapType  {
        let mut dmap = MapType::new();
        let mut q: VecDeque<(isize, isize, u16)> = VecDeque::new();
        let neigh = &[(1,0), (-1,0), (0,1), (0,-1)];        

        q.push_back((start.0, start.1, 0));
        while let Some((y,x,d)) = q.pop_front() {
            dmap.insert((y,x), d);
            for (dy,dx) in neigh{
                let cand = (y+dy, x+dx);
                let add = match self.map.get(&cand) {
                    Some(&c) => if c==0 {
                                    if !dmap.contains_key(&cand) { true }
                                    else if *dmap.get(&cand).unwrap() > (d+1) { true }
                                    else { false }
                                } 
                                else { false },
                    _ => { println!("Map element {:?} not found", cand); false }
                };
                if add && !q.contains(&(cand.0, cand.1, d+1)) { q.push_back((cand.0, cand.1, d+1)); }
            }
        }
        return dmap;
    }
}

fn main() -> Result<()>
{
    let dat = include_str!("Day15.txt");
    let cave = Cave::new(&dat)?;

    println!("{:?}", cave.units);
    // println!("{:?}", cave.map);

    Ok(())
}