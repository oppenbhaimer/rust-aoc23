use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_11")?;
    let reader = io::BufReader::new(file);

    // change to 2 for part 1
    let expansion = 1000000;
    let grid : Vec<Vec<char>> = reader.lines().map(|x| x.unwrap().chars().collect()).collect();
    let mut row_add : Vec<i32> = vec![0; grid.len()];
    let mut col_add : Vec<i32> = vec![0; grid[0].len()];
    let mut ans : i64 = 0;

    for (i, line) in grid.iter().enumerate() {

        let mut empty = true;
        for c in line {
            if *c != '.' {
                empty = false;
                break;
            }
        }

        if empty {
            row_add[i] = 1;
        }
    }

    for i in 1..row_add.len() {
        row_add[i] = row_add[i-1] + row_add[i];
    }

    for j in 0..grid[0].len() {

        let mut empty = true;
        for i in 0..grid.len() {
            if grid[i][j] != '.' {
                empty = false;
                break;
            }
        }

        if empty {
            col_add[j] = 1;
        }
    }

    for j in 1..col_add.len() {
        col_add[j] = col_add[j-1] + col_add[j];
    }

    // get galaxies
    let mut galaxies : Vec<(i32, i32)> = vec![];

    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '.' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let (y1, x1) = galaxies[i];
            let (y2, x2) = galaxies[j];
            ans += ( (y1-y2).abs() + (row_add[y1 as usize]-row_add[y2 as usize]).abs() * (expansion-1)
                 + (x1-x2).abs() + (col_add[x1 as usize]-col_add[x2 as usize]).abs() * (expansion-1) )as i64;
        }
    }

    println!("{}", ans);

    Ok(())
}
