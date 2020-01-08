fn react(polymer: &str) -> String
{
    let mut stack: Vec<char> = Vec::new();
    polymer.chars().for_each(|c| 
        match stack.last() {
            None => stack.push(c),
            Some(&lc) => if lc!=c && lc.eq_ignore_ascii_case(&c) {
                            stack.pop();
                        }
                        else {
                            stack.push(c);
                        }
        }
    );
    return stack.iter().collect();
}

fn part2(polymer: &str) -> u32
{
    return (b'a'..=b'z').map(|c| {
            let removed: String = polymer.chars().filter(|a| !(c as char).eq_ignore_ascii_case(&a) ).collect();
            return react(&removed).len();  
        })
        .min()
        .unwrap() as u32;
}

fn main()
{
    let dat = include_str!("Day05.txt").trim();
    let _t1: &'static str = "dabAcCaCBAcCcaDA";

    assert_eq!(react(&_t1),"dabCBAcaDA");
    assert_eq!(part2(&_t1), 4);
    println!("Part 1: {}", react(&dat).len());
    println!("Part 2: {}", part2(&dat));
}