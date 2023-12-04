use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_3")?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    let mut grid: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let l = line.unwrap().chars().collect();
        grid.push(l);
    }

    for (i, line) in grid.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            let sj = j;
            let mut ej = j;
            let mut num = false;
            while j < line.len() && line[j].is_digit(10) {
                ej = j;
                num = true;
                j += 1;
            }

            if !num {
                j += 1;
                continue;
            }

            let mut is_part : bool = false;
            // search for a symbol in the surrounding area
            let mut i_min = i;
            if i > 0 {
                i_min = i-1;
            }
            let mut j_min = sj;
            if sj > 0 {
                j_min = sj-1;
            }
            for ii in i_min ..= i+1 {
                for jj in j_min ..= ej+1 {
                    if ii < grid.len() && jj < line.len() {
                        if grid[ii][jj] != '.' && !grid[ii][jj].is_digit(10) {
                            is_part = true;
                            break;
                        }
                    }
                }
            }

            if is_part {
                let s = String::from_iter(grid[i][sj ..= ej].iter());
                println!("{} - {},{}", s, sj, ej);
                ans += s.parse::<i32>().unwrap();
            }
            j = ej + 1;
        }
    }

    println!("{}", ans);

    Ok(())
}
