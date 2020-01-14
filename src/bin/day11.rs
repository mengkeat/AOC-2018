
const SNUM: i32 = 3463;

fn get_power(x: i32, y: i32) -> i32
{
    let rack_id = x+10;
    let pow = (rack_id*y + SNUM)*rack_id;
    return ((pow/100)%10)-5 ;
}

fn get_max(grid: &Vec<Vec<i32>>) -> (u32, u32)
{
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;
    let mut max_sum: i32 = -100;
    for y in 3..=300 {
        for x in 3..=300 {
            let curr_sum = grid[y][x] + grid[y-3][x-3] - grid[y][x-3] - grid[y-3][x];
            if curr_sum > max_sum {
                max_x = x as u32;
                max_y = y as u32;
                max_sum = curr_sum;
            }
        }
    }
    return (max_x-2, max_y-2);
}

fn main()
{
    let mut grid = vec![ vec![0 as i32; 301]; 301];

    for y in 1..=300 {
        for x in 1..=300 {
            grid[y][x] = grid[y-1][x] + grid[y][x-1] 
                        - grid[y-1][x-1] + get_power(x as i32 ,y as i32);
        }
    }
    let (mx, my) = get_max(&grid);
    println!("Part 1 Maximum coordinate: {}, {}", mx, my);
}