use std::fs::File;
use std::io::{self, BufRead};

fn cycle(grid: &mut Vec<Vec<char>>) {

    let n = grid.len();
    let m = grid[0].len();

    // north
    for j in 0..m {
        let mut i : i32 = (n-1) as i32;
        while i >= 0 {
            let i_u = i as usize;
            if grid[i_u][j] == 'O' {
                // push tail O's up till hitting a rock
                let mut k = 1;
                while i_u+k < n && grid[i_u+k][j] == 'O' {
                    k += 1;
                }

                let mut d : i32 = 1;
                while i-d >= 0 && grid[(i-d) as usize][j] == '.' {
                    d += 1;
                }

                d -= 1;
                let d = d as usize;
                for t in 0..k {
                    let ch = grid[i_u+t-d][j];
                    grid[i_u+t-d][j] = grid[i_u+t][j];
                    grid[i_u+t][j] = ch;
                }
                i -= (d+1) as i32;
            }
            else {
                i -= 1;
            }
        }
    }

    // west
    for i in 0..n {
        let mut j : i32 = (m-1) as i32;
        while j >= 0 {
            let j_u = j as usize;
            if grid[i][j_u] == 'O' {
                let mut k = 1;
                while j_u+k < n && grid[i][j_u+k] == 'O' {
                    k += 1;
                }

                let mut d : i32 = 1;
                while j-d >= 0 && grid[i][(j-d) as usize] == '.' {
                    d += 1;
                }

                d -= 1;
                let d = d as usize;
                for t in 0..k {
                    let ch = grid[i][j_u+t-d];
                    grid[i][j_u+t-d] = grid[i][j_u+t];
                    grid[i][j_u+t] = ch;
                }
                j -= (d+1) as i32;
            }
            else {
                j -= 1;
            }
        }
    }

    // south
    for j in 0..m {
        let mut i : i32 = 0;
        while i < (n as i32) {
            let i_u = i as usize;
            if grid[i_u][j] == 'O' {
                // push tail O's up till hitting a rock
                let mut k : i32 = 1;
                while i-k >= 0 && grid[(i-k) as usize][j] == 'O' {
                    k += 1;
                }

                let mut d : i32 = 1;
                while i+d < (n as i32) && grid[(i+d) as usize][j] == '.' {
                    d += 1;
                }

                d -= 1;
                let d = d as usize;
                for t in 0..(k as usize) {
                    let ch = grid[i_u-t+d][j];
                    grid[i_u-t+d][j] = grid[i_u-t][j];
                    grid[i_u-t][j] = ch;
                }
                i += (d+1) as i32;
            }
            else {
                i += 1;
            }
        }
    }
    
    // east
    for i in 0..n {
        let mut j : i32 = 0;
        while j < (m as i32) {
            let j_u = j as usize;
            if grid[i][j_u] == 'O' {
                // push tail O's up till hitting a rock
                let mut k : i32 = 1;
                while j-k >= 0 && grid[i][(j-k) as usize] == 'O' {
                    k += 1;
                }

                let mut d : i32 = 1;
                while j+d < (n as i32) && grid[i][(j+d) as usize] == '.' {
                    d += 1;
                }

                d -= 1;
                let d = d as usize;
                // shift k elements up by d 
                for t in 0..(k as usize) {
                    let ch = grid[i][j_u-t+d];
                    grid[i][j_u-t+d] = grid[i][j_u-t];
                    grid[i][j_u-t] = ch;
                }
                j += (d+1) as i32;
            }
            else {
                j += 1;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_14")?;
    let reader = io::BufReader::new(file);

    let mut grid : Vec<Vec<char>> = reader.lines().map(|x| x.unwrap().chars().collect()).collect();
    let n = grid.len();
    let m = grid[0].len();
    let mut ans : i64 = 0;

    let mut prev_grids : Vec<Vec<Vec<char>>> = vec![];

    let mut matches = false;
    let mut idx = 0;
    let mut match_cyc = 0;

    while !matches {
        prev_grids.push(grid.clone());
        cycle(&mut grid);

        for (i, prev_grid) in prev_grids.iter().enumerate() {
            matches = true;
            for i in 0..n {
                for j in 0..m {
                    if grid[i][j] != prev_grid[i][j] {
                        matches = false;
                        break;
                    }
                }
                if !matches {
                    break;
                }
            }
            if matches {
                match_cyc = i;
                break;
            }
        }

        idx += 1;
    }

    let cyc_len = idx - match_cyc;

    println!("{}", cyc_len);

    let n_cyc_left = (1_000_000_000 - match_cyc) % cyc_len;

    let mut grid = &mut prev_grids[match_cyc];

    for _ in 0..n_cyc_left {
        cycle(&mut grid);
    }

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'O' {
                ans += (n-i) as i64;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
