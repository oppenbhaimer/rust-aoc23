use std::fs::File;
use std::cmp::min;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_13")?;
    // let file = File::open("test4")?;
    let reader = io::BufReader::new(file);

    let allgrids : Vec<Vec<char>> = reader.lines().map(|x| x.unwrap().chars().collect()).collect();
    let mut tgrid: Vec<Vec<char>> = vec![];
    let mut grids: Vec<Vec<Vec<char>>> = vec![];
    for i in 0..allgrids.len() {
        if allgrids[i].len() == 0 {
            grids.push(tgrid);
            tgrid = vec![];
        }
        else {
            tgrid.push(allgrids[i].clone());
        }
    }
    grids.push(tgrid);

    let mut ans = 0;

    for grid in grids {

        let n = grid.len();
        let m = grid[0].len();

        // find horizontal reflection line
        let mut h_reflect = 0;
        for i in 0..n-1 {
            let mut smudge_ctr = 0;
            for k in 0..min(i+1,n-i-1) {
                for j in 0..m {
                    if grid[i+k+1][j] != grid[i-k][j] {
                        smudge_ctr += 1;
                    }
                }
            }
            if smudge_ctr == 1 { // 0 {
                h_reflect = i+1;
                break;
            }
        }

        if h_reflect > 0 {
            ans += 100*h_reflect;
            continue;
        }
        
        // find vertical reflection line
        let mut v_reflect = 0;
        for j in 0..m-1 {
            let mut smudge_ctr = 0;
            for k in 0..min(j+1,m-j-1) {
                for i in 0..n {
                    if grid[i][j+k+1] != grid[i][j-k] {
                        smudge_ctr += 1;
                    }
                }
            }
            if smudge_ctr == 1 { // > 0 {
                v_reflect = j+1;
                break;
            }
        }

        ans += 100*h_reflect + v_reflect;
    }

    println!("{}", ans);

    Ok(())
}
