#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
// use std::collections::HashMap;
use core::str::FromStr;
// use std::result;

#[derive(Debug)]
struct Claim 
{
    id: u32,
    x: u32, 
    y: u32,
    w: u32, 
    h: u32
}

impl FromStr for Claim 
{
    type Err = &String;

    fn from_str(s: &str) -> Result<Claim, &String> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"
                \#
                (?P<id>[0-9]+)
                \s+@\s+
                (?P<x>[0-9]+),(?P<y>[0-9]+):\s+
                (?P<w>[0-9]+)x(?P<h>[0-9]+)
            ").unwrap();
        }

        let caps = RE.captures(s)?;
        Ok(Claim{
            id: caps["id"].parse()?,
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            w: caps["w"].parse()?,
            h: caps["h"].parse()?
        })
    }
}

fn parse_claims(s: &str) -> Vec<Claim>
{
    let mut claim_vec: Vec<Claim> = vec![];
    for line in s.lines() {
        let c = line.parse()?;
        claim_vec.push(c);
    }
    claim_vec
}

fn main()
{
    let dat = include_str!("Day03.txt");
    let claims = parse_claims(dat);
}