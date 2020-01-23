use std::collections::{BTreeMap, VecDeque};
use std::error::Error;
use std::result;
use std::cmp;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<dyn Error>>;
type MapCoord = (isize, isize);
type MapType = BTreeMap<MapCoord, u16>;

static NEIGH_D: &'static [(isize, isize)] = &[(1,0), (-1,0), (0,1), (0,-1)];

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Race { Elf, Goblin }

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Unit {
    race: Race,
    x: isize,
    y: isize,
    attack: i16,
    hp: i16,
    killed: bool,
}

impl Unit {
    fn coords_in_range(&self) -> Vec<MapCoord> {
        NEIGH_D.iter()
            .map(|(dy,dx)|  (self.y+dy, self.x+dx) )
            .collect()
    }
}

impl Ord for Unit {
    fn cmp(&self, rhs: &Unit) -> cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, rhs: &Unit) -> Option<cmp::Ordering> {
        Some((self.y, self.x).cmp(&(rhs.y, rhs.x)))
    }
}

#[derive(Debug)]
struct Cave {
    units: BTreeMap<MapCoord, Unit>,
    map: MapType,
    orig_elves_count: u16,
}

impl Cave {

    fn new(dat: &str) -> Result<Self> {
        let mut cave = Cave {units: BTreeMap::new(), map: MapType::new(), orig_elves_count: 0 };
        for (y, row_str) in dat.lines().enumerate() {
            for (x, ele) in row_str.char_indices() {
                let _x = x as isize;
                let _y = y as isize;
                let map_ele = match ele {
                    'E' => { cave.units.insert( (_y, _x), Unit{race: Race::Elf, x: _x, y: _y, attack: 3, hp: 200, killed: false}); 
                        cave.orig_elves_count += 1; 0
                    },
                    'G' => { cave.units.insert( (_y, _x), Unit{race: Race::Goblin, x: _x, y: _y, attack: 3, hp: 200, killed: false}); 0 },
                    '.' => 0,
                    '#' => 1,
                    _ => { return err!("Error reading map in from data file!"); }
                };
                cave.map.insert((_y, _x), map_ele);
            }
        }
        Ok(cave)
    }

    fn set_elves_attack(&mut self, a: i16) {
        for (_, unit) in self.units.iter_mut() {
            if unit.race == Race::Elf {
                unit.attack = a;
            }
        }
    }

    fn distance_map(&self, start: &MapCoord, dmap: &mut MapType) {
        let mut q: VecDeque<(isize, isize, u16)> = VecDeque::new();
        q.push_back((start.0, start.1, 0));
        while let Some((y,x,d)) = q.pop_front() {
            dmap.insert((y,x), d);
            for (dy,dx) in NEIGH_D {
                let cand = (y+dy, x+dx);
                if self.units.contains_key(&cand) { continue; }
                let add = match self.map.get(&cand) {
                    Some(&c) => 
                        if c==0 {
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
    }

    // Given src and tgt units, computes the valid adjacent (distance, in_range_coord)
    fn get_valid_adj(&self, src: &MapCoord, tgt: &MapCoord) -> Vec<(u16, MapCoord)> {
        let tgt_unit = self.units[&tgt];
        let mut dist_to_src = MapType::new();
        self.distance_map(src, &mut dist_to_src);

        tgt_unit.coords_in_range().iter()
            .filter_map(|c| dist_to_src.get(&c).and_then(|d| Some((*d, *c))) )
            .collect()
    }

    // Returns the coordinate in range and the associated chosen unit's coordinate
    fn nearest_in_range(&self, a: &Unit) -> Option<MapCoord> {
        let cand_targets: Vec<(u16, MapCoord)> = self.units.iter()
            .filter(|(_tgt_c, tgt_u)| a.race!=tgt_u.race)   // tgts are different race
            .map(|(tgt_c, _tgt_u)| self.get_valid_adj(&(a.y, a.x), tgt_c))
            .flatten().collect();
        // println!("Candidate tgts: {:?}", cand_targets);
        let (_, min_coord) = cand_targets.iter().min()?;
        Some(*min_coord)
    }

    // Moves unit from src_coord towards dst_coord
    fn unit_next(&self, src_coord: &MapCoord, dst_coord: &MapCoord) -> Option<MapCoord> {
        if src_coord == dst_coord { 
            return Some(*src_coord); 
        }
        let mut dist_to_dst = MapType::new();
        self.distance_map(dst_coord, &mut dist_to_dst);
        let (_min_dist, min_coord) = NEIGH_D.iter()
            .filter_map(|(dy,dx)| {
                let c = (src_coord.0+dy, src_coord.1+dx);
                if self.units.contains_key(&c) { return None; }
                dist_to_dst.get(&c).map(|d| (d,c) )
            })
            .min()?;
        return Some(min_coord);
    }

    fn move_unit(&mut self, unit_coord: &MapCoord, next_coord: &MapCoord)  {
        let mut new_unit = self.units[&unit_coord].clone();
        new_unit.x = next_coord.1;
        new_unit.y = next_coord.0;
        self.units.remove(&unit_coord);
        self.units.insert(*next_coord, new_unit);
    }

    fn attack_from(&mut self, c: &MapCoord) {
        let u = self.units[&c];
        if let Some((_, tgt_c)) = u.coords_in_range().iter()
            .filter_map(|c| self.units.get(&c)
                                .and_then(|tgt| 
                                    if u.race!=tgt.race { Some((tgt.hp, *c)) }
                                    else { None } ) 
            )
            .min() 
        {
            let mut tgt_unit = self.units.get_mut(&tgt_c).unwrap();
            tgt_unit.hp -= u.attack;
            if tgt_unit.hp <= 0 {
                // println!("Unit at {:?} killed", tgt_c);
                self.units.remove(&tgt_c);
            }
            // else {
            //     println!("Unit at {:?} attack {:?}", c, tgt_c);
            // }
        }
    }

    fn total_victory(&self) -> bool {
        let f: &Unit = self.units.values().nth(0).unwrap();
        self.units.values().all(|&u| u.race==f.race )
    }

    fn all_elves_survive(&self) -> bool {
        self.units.values().filter(|&u| u.race==Race::Elf).count() == self.orig_elves_count as usize
    }

    fn print_map(&self) {
        let (_, max_x) = self.map.keys().max_by_key(|c| c.1).unwrap();
        println!();
        for (c, v) in self.map.iter() {
            if self.units.contains_key(c) {
                match self.units[c].race  {
                    Race::Elf => print!("E"),
                    Race::Goblin => print!("G")
                }
            }
            else {
                match v {
                    0 => print!("."),
                    1 => print!("#"),
                    _ => print!("?"), 
                }
            }
            if c.1==*max_x { println!(); }
        }
        println!();
    }

    // Returns true if no total victory and has to continue battle
    fn next(&mut self) -> bool {
        let all_coords: Vec<_> = self.units.keys().cloned().collect();

        // self.print_map();

        for unit_coord in all_coords {
            if !self.units.contains_key(&unit_coord) { continue; }
            let src_unit = self.units[&unit_coord];

            // println!("Processing for unit at coord {:?}  hp: {}", unit_coord, src_unit.hp);
            // println!("-------------------------------------------------------");

            // Find nearest attack pos in range
            let tgt_inrange_coord = self.nearest_in_range(&src_unit);
            if tgt_inrange_coord.is_none() { continue; }
            // println!("Target in range {:?} ", tgt_inrange_coord);

            // Make move if necessary
            let next_coord = self.unit_next(&unit_coord, &tgt_inrange_coord.unwrap());
            next_coord.map(|c| self.move_unit(&unit_coord, &c));
            // println!("Moving to next coordinate: {:?}", next_coord);

            // Perform attack if in range
            next_coord.map(|c| self.attack_from(&c));
            // println!("");
        }
        return !self.total_victory();
    }

    fn hit_points_sum(&self) -> i16 {
        self.units.iter().fold(0, |acc, (&_, &u)| acc + u.hp)
    }
}

fn main() -> Result<()>
{
    let dat = include_str!("Day15.txt");
    let mut cave = Cave::new(&dat)?;
    let mut count: u32 = 0;

    while cave.next() { 
        // println!("==================== Round: {}", count+1);
        count += 1;
    };
    let prod = cave.hit_points_sum();
    println!("Number of rounds: {}, hitpoints: {}, product: {}", count, prod, count*prod as u32);

    for e_pow in 4..100 {
        let mut c = Cave::new(&dat)?;
        let mut rnds: u32 = 0;
        println!("Elf power {}", e_pow);
        c.set_elves_attack(e_pow);
        while c.next() { rnds += 1; };
        if c.all_elves_survive() {
            let p = c.hit_points_sum();
            println!("Part 2 rounds: {}, hitpoints: {}, product: {} ", rnds, p, rnds*p as u32);
            break
        }
    }

    Ok(())
}