use std::fs::File;
use std::io::{self, BufRead};

enum Direction {
    Right,
    Left,
    Up,
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_15")?;
    let mut reader = io::BufReader::new(file);

    let grid: Vec<Vec<i32>> = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|y| (y as i32) - ('0' as i32))
                .collect()
        })
        .collect();
    let n = grid.len();
    let m = grid[0].len();
    let dp: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    let dir: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    let mut ans: i32 = 0;

    for i in (1..=n).rev() {
        for j in (1..=m).rev() {}
    }

    let println!("{}", ans);

    Ok(())
}
