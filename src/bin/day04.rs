// #[macro_use]
// extern crate lazy_static;
extern crate regex;

use regex::Regex;
// use std::collections::HashMap;
// use core::str::FromStr;
// use std::result;
// use std::error::Error;

// type Result<T> = result::Result<T, Box<dyn Error>>;

// #[derive(Debug)]
// enum LogType { Start, Sleep, Wake }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct LogEntry
{
    month: i8,
    day: i8,
    hour: i8,
    min: i8,
    id: i16,
    sleep: bool
}


fn main()
{
    let dat = include_str!("Day04.txt");
    let re = Regex::new(r"(?x)
        \[ 1518 - (?P<month>\d+) - (?P<day>\d+) \s (?P<hour>\d+) : (?P<min>\d+) \] \s
        (?:Guard\s\#)? (?P<type>(falls|wakes|\d+))
        ").unwrap();

    let mut guard_logs = Vec::new();
    for cap in re.captures_iter(dat) {
        let id: i16 = match &cap["type"] {
            "falls"|"wakes" => -1,
            num => num.parse().unwrap()
        };
        guard_logs.push(LogEntry{
            month: cap["month"].parse().unwrap(),
            day: cap["day"].parse().unwrap(),
            hour: cap["hour"].parse().unwrap(),
            min: cap["min"].parse().unwrap(),
            id: id, 
            sleep: match &cap["type"] {
                "falls"=> true,
                _ => false
            }
        });
    }
    guard_logs.sort();  

    println!("{:?}", guard_logs);
}