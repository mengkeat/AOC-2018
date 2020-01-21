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
}

impl Cave {

    fn new(dat: &str) -> Result<Self> {
        let mut cave = Cave {units: BTreeMap::new(), map: MapType::new() };
        for (y, row_str) in dat.lines().enumerate() {
            for (x, ele) in row_str.char_indices() {
                let _x = x as isize;
                let _y = y as isize;
                let map_ele = match ele {
                    'E' => { cave.units.insert( (_y, _x), Unit{race: Race::Elf, x: _x, y: _y, attack: 3, hp: 200, killed: false}); 0 },
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

    fn distance_map(&self, start: &MapCoord) -> MapType  {
        let mut dmap = MapType::new();
        let mut q: VecDeque<(isize, isize, u16)> = VecDeque::new();

        q.push_back((start.0, start.1, 0));
        while let Some((y,x,d)) = q.pop_front() {
            dmap.insert((y,x), d);
            for (dy,dx) in NEIGH_D {
                let cand = (y+dy, x+dx);
                // if self.units.contains_key(&cand) { continue; }
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

    // Given src and tgt units, computes the valid adjacent (distance, in_range_coord)
    // fn get_valid_adj(&self, src: &MapCoord, tgt: &MapCoord) -> Vec<(u16, MapCoord)> {
    //     let tgt_unit = self.units[&tgt];
    //     tgt_unit.coords_in_range().iter()
    //         .
    // }

    // Returns the coordinate in range and the associated chosen unit's coordinate
    fn nearest_in_range(&self, a: &Unit) -> Option<(MapCoord, MapCoord)> {
        let mut cand_targets: Vec<(u16, MapCoord, MapCoord)> = Vec::new();
        let dist_to_a = self.distance_map(&(a.y, a.x));

        for (tgt_coord, tgt_unit) in self.units.iter() {
            if a.race==tgt_unit.race { continue; }
            for c in tgt_unit.coords_in_range() {
                if dist_to_a.contains_key(&c) && !self.units.contains_key(&c) {
                    cand_targets.push( (dist_to_a[&c], c, *tgt_coord));
                }
            }
        }
        // self.units.iter()
        //     .filter(|tgt_c, tgt_u| a.race!=u.race)   // tgts are different race
        //     .map(|tgt_c, tgt_u| 
        //     )

        // println!("Candidate tgts: {:?}", cand_targets);
        let (_, min_coord, tgt_coord) = cand_targets.iter().min()?;
        Some((*min_coord, *tgt_coord))
    }

    // Moves unit from src_coord towards dst_coord
    fn unit_next(&self, src_coord: &MapCoord, dst_coord: &MapCoord) -> Option<MapCoord> {
        if src_coord == dst_coord { 
            return Some(*src_coord); 
        }
        let dist_to_dst = self.distance_map(dst_coord);
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

    fn attack(&mut self, src: &MapCoord, dst: &MapCoord) {
        let attack = self.units[dst].attack;
        if NEIGH_D.contains(&(dst.0-src.0, dst.1-src.1)) {
            let dst_unit = self.units.get_mut(&dst).unwrap();
            dst_unit.hp -= attack;
            if dst_unit.hp < 0 { 
                println!("Unit at {:?} killed", dst);
                self.units.remove(dst);
            }
        }        
    }

    fn next(&mut self) -> Option<()> {
        let all_coords: Vec<_> = self.units.keys().cloned().collect();
        for unit_coord in all_coords {
            if !self.units.contains_key(&unit_coord) { continue; }

            println!("Processing for unit at coord {:?}", unit_coord);
            println!("------------------------------------");
            let src_unit = self.units[&unit_coord];
            // Find nearest attack pos in range
            let (tgt_inrange_coord, tgt_coord) = self.nearest_in_range(&src_unit)?;
            println!("Target in range {:?} of target coord: {:?}", tgt_inrange_coord, tgt_coord);

            // Make move if necessary
            let next_coord = self.unit_next(&unit_coord, &tgt_inrange_coord);
            next_coord.map(|c| self.move_unit(&unit_coord, &c));
            println!("Moving to next coordinate: {:?}", next_coord);

            // Perform attack if in range
            next_coord.map(|c| self.attack(&c, &tgt_coord));
            println!("");
        }
        return Some(());
    }

    fn hit_points_sum(&self) -> i16 {
        self.units.iter().fold(0, |acc, (&_, &u)| acc + u.hp)
    }
}

fn main() -> Result<()>
{
    let dat = include_str!("Day15-test1.txt");
    let mut cave = Cave::new(&dat)?;
    let mut count: u32 = 0;

    while let Some(_) = cave.next() { 
        println!("==================== Round: {}", count+1);
        count += 1;
    };
    let prod = cave.hit_points_sum();
    println!("Number of rounds: {}, hitpoints: {}, product: {}", count, prod, count*prod as u32);

    Ok(())
}