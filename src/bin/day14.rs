use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_14")?;
    let reader = io::BufReader::new(file);

    let mut grid : Vec<Vec<char>> = reader.lines().map(|x| x.unwrap().chars().collect()).collect();
    let n = grid.len();
    let m = grid[0].len();
    let mut ans : i64 = 0;

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
                // println!("{} {} {} {}", j, i, k, d);
                // shift k elements up by d 
                for t in 0..k {
                    let ch = grid[i_u+t-d][j];
                    grid[i_u+t-d][j] = grid[i_u+t][j];
                    grid[i_u+t][j] = ch;
                }
                /*
                for row in grid.iter() {
                    for ch in row {
                        print!("{}", ch);
                    }
                    println!("");
                }*/
                i -= (d+1) as i32;
            }
            else {
                i -= 1;
            }
        }
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
