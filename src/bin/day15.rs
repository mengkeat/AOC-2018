use std::collections::{BTreeMap, HashMap};

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

type MapType = BTreeMap<(usize, usize), char>;

#[derive(Debug)]
struct Cave {
    units: Vec<Unit>,
    map: MapType,
    dist_map_cache: HashMap<(usize, usize), MapType>,   
}

impl Cave {

    fn new(dat: &str) -> Self {
        let mut cave = Cave {units: Vec::new(), map: MapType::new(), dist_map_cache: HashMap::new() };
        for (y, row_str) in dat.lines().enumerate() {
            for (x, ele) in row_str.char_indices() {
                let map_ele = match ele {
                    'E' => { cave.units.push( Unit{race: Race::Elf, x: x, y: y, attack: 3, hp: 200, killed: false}); '.' },
                    'G' => { cave.units.push( Unit{race: Race::Goblin, x: x, y: y, attack: 3, hp: 200, killed: false}); '.' },
                    x => x ,
                };
                cave.map.insert((y,x), map_ele);
            }
        }
       return cave; 
    }

}

fn main()
{
    let dat = include_str!("Day15.txt");
    let mut cave = Cave::new(&dat);

    println!("{:?}", cave.units);
    println!("{:?}", cave.map);
}