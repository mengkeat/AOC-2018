// #[macro_use]
// extern crate lazy_static;
// extern crate regex;

// use regex::Regex;
// use std::collections::HashMap;
use core::str::FromStr;
use std::result;
use std::error::Error;

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum LogEntry
{
    NewGuard(u16), Wake(u8), Sleep(u8)
}

impl FromStr for LogEntry 
{
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<LogEntry>
    {
        if s.contains("Guard") { 
            let pos = match s.find(" b") { Some(x) => x, _ => Err("Error parsing Gaurd #")? };
            return Ok(LogEntry::NewGuard(s[26..pos].parse()?)) 
        }

        let min: u8 = s[15..17].parse()?;
        if s.contains("falls") { return Ok(LogEntry::Wake(min)); }
        if s.contains("wakes") { return Ok(LogEntry::Sleep(min)); }
        return Err("Cannot parse LogEntry")?;
    }
}

fn main()
{
    let dat = include_str!("Day04.txt");
    let mut dat_vec: Vec<&str> = dat.lines().collect();
    dat_vec.sort();

    let logs: Vec<LogEntry> = dat_vec.iter().map(|s| s.parse().unwrap() ).collect();
    for d in logs {
        println!("{:?}", d);
    }
}