use std::fs::File;
use std::io::{self, BufRead};

fn get_reachable(grid: &Vec<Vec<char>>, start: (i32, i32), steps: i32) -> i64 {

    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let mut dists: Vec<Vec<Option<i32>>> = vec![vec![None; m as usize]; n as usize];
    let (sx, sy) = start;
    // let (sx, sy) = (5, 5);

    let mut s = vec![];
    s.push((sx, sy));
    dists[sy as usize][sx as usize] = Some(0);
    let mut dist = 1;
    while !s.is_empty() {
        let mut frontier = vec![];
        while !s.is_empty() {
            let (x, y) : (i32, i32) = s.pop().unwrap();
            if (x+1) < m && dists[y as usize][((x+1)) as usize].is_none() && grid[y as usize][(x+1) as usize] == '.' {
                dists[y as usize][(x+1) as usize] = Some(dist);
                frontier.push(((x+1),y));
            }
            if (y+1) < n && dists[(y+1) as usize][x as usize].is_none() && grid[(y+1) as usize][x as usize] == '.' {
                dists[(y+1) as usize][x as usize] = Some(dist);
                frontier.push((x,(y+1)));
            }
            if x-1 >= 0 && dists[y as usize][(x-1) as usize].is_none() && grid[y as usize][(x-1) as usize] == '.' {
                dists[y as usize][(x-1) as usize] = Some(dist);
                frontier.push(((x-1),y));
            }
            if y-1 >= 0 && dists[(y-1) as usize][x as usize].is_none() && grid[(y-1) as usize][x as usize] == '.' {
                dists[(y-1) as usize][x as usize] = Some(dist);
                frontier.push((x,y-1));
            }
        }
        dist += 1;
        for p in frontier {
            s.push(p);
        }
    }

    // count outward in L1 circles
    //    #
    //   # #
    //  # S #
    //   # #
    //    #
    let n_steps = steps;
    let mut tot_reachable_squares = 0;

    for row in dists {
        for cell in row {
            match cell {
                Some(n) => {
                    if n%2 == n_steps%2 && n <= n_steps {
                        tot_reachable_squares += 1;
                        // print!("O");
                    }
                    else {
                        // print!(".");
                    }
                },
                None => {
                    // print!("#"); 
                }
            }
        }
        // println!("");
    }

    tot_reachable_squares
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_21_expanded")?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader.lines().map(|x| x.unwrap().chars().collect()).collect();

    let mut ans: i64 = 0;

    // part 1
    ans = get_reachable(&grid, (131*2+65, 131*2+65), 64);
    println!("{}", ans);
    ans = get_reachable(&grid, (131*2+65, 131*2+65), 65);
    println!("{}", ans);
    ans = get_reachable(&grid, (131*2+65, 131*2+65), 131+65);
    println!("{}", ans);
    ans = get_reachable(&grid, (131*2+65, 131*2+65), 131*2+65);
    println!("{}", ans);
    ans = get_reachable(&grid, (131*2+65, 131*2+65), 131*3+65);
    println!("{}", ans);

    // fit a quadratic on these after this.

    Ok(())
}
