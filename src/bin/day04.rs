// #[macro_use]
// extern crate lazy_static;
// extern crate regex;

// use regex::Regex;
use std::collections::HashMap;
use core::str::FromStr;
use std::result;
use std::error::Error;

type Result<T> = result::Result<T, Box<dyn Error>>;
type HistType = HashMap<u16, [u16; 60]>;

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
        if s.contains("falls") { return Ok(LogEntry::Sleep(min)); }
        if s.contains("wakes") { return Ok(LogEntry::Wake(min)); }
        return Err("Cannot parse LogEntry")?;
    }
}

fn construct_hist(log: &Vec<LogEntry>) -> HistType
{
    let mut hist: HashMap<u16, [u16; 60]> = HashMap::new();
    let mut curr_id: u16 = 0;
    let mut sleep_time: u8 = 0;

    log.iter().for_each(|entry|
        match entry {
            LogEntry::NewGuard(id) => curr_id = *id,
            LogEntry::Sleep(min) => sleep_time = *min,
            LogEntry::Wake(min) => 
                for t in sleep_time .. *min {
                    hist.entry(curr_id).or_insert([0u16; 60])[t as usize] += 1;
                }
        }
    );
    return hist;
}

fn part1(hist: &HistType) -> u32 
{
    let (id, _) = hist.iter().max_by_key(|(_, h)| h.iter().sum::<u16>()).unwrap();
    let (m, _) = hist[id].iter().enumerate().max_by_key(|(_, c)| **c).unwrap();
    return (m as u32) * (*id as u32);
}

fn main()
{
    let dat = include_str!("Day04.txt");
    let mut dat_vec: Vec<&str> = dat.lines().collect();
    dat_vec.sort();

    let logs: Vec<LogEntry> = dat_vec.iter().map(|s| s.parse().unwrap() ).collect();
    let hist = construct_hist(&logs);

    println!("Part 1: {}", part1(&hist));
}