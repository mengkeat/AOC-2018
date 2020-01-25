use std::io::{self, Read};
use std::error::Error;
use std::result;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}
type Result<T> = result::Result<T, Box<dyn Error>>;

// 8-neighbours (dx,dy)
// static NEIGH: &'static [(isize, isize)] = 
//     [ (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1) ];

#[derive(Debug, Clone)]
enum Acre { Open, Tree, Lumberyard}

#[derive(Debug, Clone)]
struct Area {
    map: Vec<Vec<Acre>>,
}

impl Area {

    fn new(dat: &str) -> Result<Area> {
        let mut map: Vec<Vec<Acre>> = Vec::new();
        for line in dat.lines() {
            let mut row: Vec<Acre> = Vec::new();
            for c in line.chars() {
                if c=='.' { row.push(Acre::Open); }
                else if c=='|' { row.push(Acre::Tree); }
                else if c=='#' { row.push(Acre::Lumberyard); }
                else { return err!("Unknown map type!"); }
            }
            map.push(row);
        }
        Ok( Area{ map: map } )
    }
}

fn main() -> Result<()> {
    let mut dat = String::new();
    io::stdin().read_to_string(&mut dat)?;
    let area = Area::new(&dat)?;

    Ok(())
}