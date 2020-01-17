use std::collections::{BTreeMap, HashMap};
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
    x: usize,
    y: usize,
    attack: i16,
    hp: i16,
    killed: bool,
}

type MapType = BTreeMap<(usize, usize), u16>;

#[derive(Debug)]
struct Cave {
    units: Vec<Unit>,
    map: MapType,
    dist_map_cache: HashMap<(usize, usize), MapType>,   
}

impl Cave {

    fn new(dat: &str) -> Result<Self> {
        let mut cave = Cave {units: Vec::new(), map: MapType::new(), dist_map_cache: HashMap::new() };
        for (y, row_str) in dat.lines().enumerate() {
            for (x, ele) in row_str.char_indices() {
                let map_ele = match ele {
                    'E' => { cave.units.push( Unit{race: Race::Elf, x: x, y: y, attack: 3, hp: 200, killed: false}); 0 },
                    'G' => { cave.units.push( Unit{race: Race::Goblin, x: x, y: y, attack: 3, hp: 200, killed: false}); 0 },
                    '.' => 0,
                    '#' => 1,
                    _ => { return err!("Error reading map in from data file!"); }
                };
                cave.map.insert((y,x), map_ele);
            }
        }
       return Ok(cave);
    }

}

fn main() -> Result<()>
{
    let dat = include_str!("Day15.txt");
    let mut cave = Cave::new(&dat)?;

    println!("{:?}", cave.units);
    // println!("{:?}", cave.map);

    Ok(())
}