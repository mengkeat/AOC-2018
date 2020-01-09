
type Pt = (i32, i32);

fn m_dist(a: Pt, b: Pt) -> i32 {  (a.0-b.0).abs() + (a.1-b.1).abs() }

fn get_points(s: &str) -> Vec<Pt>
{
    return s.lines()
     .map(|line| {
        let coord: Vec<i32> = line.split(", ").map(|w| w.trim().parse::<i32>().unwrap() ).collect();
        return (coord[0], coord[1]);
     }).collect();
}

fn part1(pts: &Vec<Pt>, x_max: i32, y_max: i32) -> i32
{
    let mut area = vec![0; pts.len()];
    for x in 0..x_max {
        for y in 0..y_max {
            let dist_to_pts: Vec<i32> = pts.iter().map(|p| m_dist(*p, (x,y)) ).collect();
            let min_dist: i32 = *dist_to_pts.iter().min().unwrap();
            let min_pts: Vec<(usize, &i32)> = dist_to_pts.iter().enumerate().filter(|p| *p.1==min_dist ).collect();
            if min_pts.len()==1 {
                let i: usize = min_pts[0].0;
                if x==0 || x==x_max-1 || y ==0 || y==y_max-1 {
                    area[i] = -1;
                }
                else {
                    area[i] += (area[i]>=0) as i32;
                }
            }
        }
    }
    return *area.iter().max().unwrap();
}

fn part2(pts: &Vec<Pt>, x_max: i32, y_max: i32) -> i32
{
    let mut count: i32 = 0;
    for x in 0..x_max {
        for y in 0..y_max {
            if pts.iter().map(|p| m_dist(*p, (x,y))).sum::<i32>() < 10000 {
                count += 1;
            }
        }
    }
    return count;
}

fn main()
{
    let dat = include_str!("Day06.txt");
    let pts = get_points(dat);

    let x_max = pts.iter().max_by_key(|p| p.0).unwrap().0;
    let y_max = pts.iter().max_by_key(|p| p.1).unwrap().1;

    println!("Part 1: {}", part1(&pts, x_max, y_max));
    println!("Part 2: {}", part2(&pts, x_max, y_max));
}