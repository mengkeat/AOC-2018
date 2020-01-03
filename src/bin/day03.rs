#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use core::str::FromStr;
use std::result;
use std::error::Error;

#[derive(Debug)]
struct Claim 
{
    id: u32,
    x: u32, 
    y: u32,
    w: u32, 
    h: u32
}

type Result<T> = result::Result<T, Box<dyn Error>>;

impl FromStr for Claim 
{
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Claim> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"(?x)
                \#
                (?P<id>[0-9]+)
                \s+@\s+
                (?P<x>[0-9]+),(?P<y>[0-9]+):\s+
                (?P<w>[0-9]+)x(?P<h>[0-9]+)
            ").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        Ok(Claim{
            id: caps["id"].parse()?,
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            w: caps["w"].parse()?,
            h: caps["h"].parse()?
        })
    }
}

fn main()
{
    let dat = include_str!("Day03.txt");
    let claims: Vec<Claim> = dat.lines().map(|l: &str| l.parse().unwrap()).collect();
    let mut m = HashMap::new();

    for c in claims {
        for x in c.x .. (c.x+c.w) {
            for y in c.y .. (c.y+c.h) {
                *m.entry((x,y)).or_insert(0) += 1;
            }
        }
    }

    println!("Part 1: {}", m.values().filter(|&&e| e>1).count());
}