#[derive(Debug)]
enum Dir { Left, Straight, Right }

#[derive(Debug)]
struct Cart {
    x: i32,
    y: i32,
    d: Dir,
    dx: i32,
    dy: i32,
    crashed: bool
}

impl Cart {
    fn turn_left(&mut self) {
        let (ax,ay) = (self.dx, self.dy);
        self.dx = ay;
        self.dy = -ax;
    }

    fn turn_right(&mut self) {
        let (ax,ay) = (self.dx, self.dy);
        self.dx = -ay;
        self.dy = ax;
    }

    // Changes the current direction of the Cart based 
    // on current direction and udpates dx, dy 
    fn crossroad_changedir(&mut self) {
        match self.d {
            Dir::Left     => { self.d = Dir::Straight; },
            Dir::Straight => { self.d = Dir::Right; self.turn_right(); },
            Dir::Right    => { self.d = Dir::Left; self.turn_left(); },
        }
    }
}

#[derive(Debug)]
struct Simul{
    carts: Vec<Cart>,
    grid: Vec<Vec<char>>
}

impl Simul {
    fn new(inp: &Vec<&str>) -> Simul { 
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut carts: Vec<Cart> = Vec::new();

        for (y, row_str) in inp.iter().enumerate() {
            let row: Vec<char> = row_str.chars()
                .enumerate()
                .map( |(x, v)| 
                    match v {
                        '>' => { carts.push(Cart{x: x as i32, y: y as i32, d: Dir::Right, dx: 1, dy: 0, crashed: false}); return '-'; }
                        '<' => { carts.push(Cart{x: x as i32, y: y as i32, d: Dir::Right, dx:-1, dy: 0, crashed: false}); return '-'; }
                        '^' => { carts.push(Cart{x: x as i32, y: y as i32, d: Dir::Right, dx: 0, dy:-1, crashed: false}); return '|'; }
                        'v' => { carts.push(Cart{x: x as i32, y: y as i32, d: Dir::Right, dx: 0, dy: 1, crashed: false}); return '|'; }
                        _ => v
                    }
                )
                .collect();
            grid.push(row);
        }
        return Simul { grid: grid, carts: carts };
    }

    // Returns index to the 2 colliding carts if any
    fn check_collide(&mut self) -> Option<(usize, usize)>  {
        let mut ret: Option<(usize, usize)> = None;

        for i in 0..self.carts.len()-1 {
            for j in i+1..self.carts.len() {
                if self.carts[i].x==self.carts[j].x && self.carts[i].y==self.carts[j].y 
                && !self.carts[i].crashed && !self.carts[j].crashed {
                    self.carts[i].crashed = true;
                    self.carts[j].crashed = true;
                    ret = Some((i,j));
                }
            }
        }
        return ret;
    }

    fn step(&mut self) -> bool {
        for c in self.carts.iter_mut() {
            c.x += c.dx;
            c.y += c.dy;
            match self.grid[c.y as usize][c.x as usize] {
                '/'  => if c.dy!=0 { c.turn_right(); } else if c.dx!=0 { c.turn_left(); }, 
                '\\' => if c.dy!=0 { c.turn_left(); } else if c.dx!=0 { c.turn_right(); },
                '+'  => { c.crossroad_changedir(); },
                '|' | '-' => {},
                _ => {},
            }
        }
        match self.check_collide() {
            Some((i,j)) => { println!("Collision {:?} {:?}", self.carts[i], self.carts[j]); return true; }, 
            _ => { return false; },
        }
    }

    fn num_carts(&self) -> usize {
        return self.carts.iter().filter(|c| !c.crashed ).count();
    }
}

fn main()
{
    let map_str: Vec<&str> = include_str!("Day13.txt").lines().collect();
    let mut s = Simul::new(&map_str);

    while s.num_carts()!=1 { s.step(); }
    for c in s.carts.iter() {
        if !c.crashed {
            println!("Last cart: {:?}", c); 
        }
    }
}