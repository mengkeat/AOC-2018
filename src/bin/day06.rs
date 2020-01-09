
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

fn main()
{
    let dat = include_str!("Day06.txt");
    let pts = get_points(dat);
    println!("{:?}", pts);

    let x_max = pts.iter().max_by_key(|p| p.0).unwrap().0;
    let y_max = pts.iter().max_by_key(|p| p.1).unwrap().1;
}