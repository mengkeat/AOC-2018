use std::collections::HashSet;

fn part1(nums: &Vec<i32>)
{
    let freq: i32 = nums.iter().sum();
    println!("Part 1: {}", freq);
}


fn part2(nums: &Vec<i32>)
{
    let mut reached = HashSet::new();
    let mut curr_freq: i32 = 0;
    let mut found = false;

    reached.insert(0);
    while !found {
        for n in nums {
            curr_freq += n;
            if reached.contains(&curr_freq) {
                found = true;
                break;
            }
            reached.insert(curr_freq);
        }
    }
    println!("Part 2: {}", curr_freq);    
}

fn main() {
    let contents = include_str!("Day01.txt");
    let nums: Vec<i32> = contents.lines().map(|s| s.parse().unwrap()).collect();

    part1(&nums);
    part2(&nums);
}
