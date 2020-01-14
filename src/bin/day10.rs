// pos: 10..16, 18..24
// vel: 36..38  40..42

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32, y: i32, dx: i32, dy: i32
}

fn parse(dat: &str) -> Vec<Point>
{
    let mut d = Vec::new();
    for line in dat.lines() {
        d.push( Point { 
            x:  line[10..16].trim().parse().unwrap(),
            y:  line[18..24].trim().parse().unwrap(),
            dx: line[36..38].trim().parse().unwrap(),
            dy: line[40..42].trim().parse().unwrap()
        });
    }    
    return d;
}

fn get_limits(pts: &Vec<Point>) -> (i32, i32, i32, i32)
{
    let min_x = pts.iter().min_by_key(|p| p.x).unwrap().x;
    let max_x = pts.iter().max_by_key(|p| p.x).unwrap().x;
    let min_y = pts.iter().min_by_key(|p| p.y).unwrap().y;
    let max_y = pts.iter().max_by_key(|p| p.y).unwrap().y;
    return (min_x, max_x, min_y, max_y);
}

fn get_area(pts: &Vec<Point>) -> i64
{
    let (min_x, max_x, min_y, max_y) = get_limits(pts);
    return (max_x-min_x).abs() as i64 * (max_y-min_y).abs() as i64;
}

fn step_points(pts: &mut Vec<Point>) 
{
    for p in pts.iter_mut() {
        p.x += p.dx;
        p.y += p.dy;
    }
}

fn get_message(pts: &mut Vec<Point>)
{
    let mut prev_a = get_area(pts);        
    let mut count: i32 = 0;

    loop {
        step_points(pts);
        count += 1;
        let curr_a = get_area(pts);
        let (min_x, max_x, min_y, max_y) = get_limits(pts);

        if curr_a < 100*100 && curr_a>prev_a {
            println!("Area: {}, count: {}", curr_a, count);
            return;
        }

        if curr_a < 100*100 {
            println!("Count: {}", count);
            let mut grid = vec![ vec![' '; (max_x-min_x+1) as usize]; (max_y-min_y+1) as usize];
            for p in pts.iter() {
                grid[(p.y-min_y) as usize][(p.x-min_x) as usize] = '#';
            }
            for y in 0..=max_y-min_y {
                let row: String = grid[y as usize].iter().collect();
                println!("{}", row);
            }
        }
        prev_a = curr_a;
    }
}

fn main()
{
    let dat = include_str!("Day10.txt");
    let all_pts = parse(dat);
    
    get_message(&mut all_pts.clone());
}