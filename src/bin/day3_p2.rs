use std::fs::File;
use std::collections::HashSet;
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

    // parse and map all the integers on the grid into an easier format 

    let mut integer_map : Vec<i32> = vec![0; 1];
    let mut int_pos_map : Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];

    let mut int_idx = 1;
    for (i, line) in grid.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            let sj = j;
            let mut ej = j;
            let mut num = false;
            while j < line.len() && line[j].is_digit(10) {
                ej = j;
                int_pos_map[i][j] = int_idx;
                num = true;
                j += 1;
            }

            if !num {
                j += 1;
                continue;
            }

            integer_map.push(String::from_iter(line[sj..=ej].iter()).parse::<i32>().unwrap());
            int_idx += 1;
        }
    }

    for (i, line) in grid.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            if grid[i][j] != '*' {
                j += 1;
                continue;
            }

            // encounter a star - make sure it's a gear first (get numbers 
            // around it)

            let mut i_min = i;
            if i > 0 {
                i_min = i-1;
            }
            let mut j_min = j;
            if j > 0 {
                j_min = j-1;
            }
            let mut nbd_nums = HashSet::new();
            for ii in i_min ..= i+1 {
                for jj in j_min ..= j+1 {
                    if ii < grid.len() && jj < line.len() && int_pos_map[ii][jj] != 0 {
                        nbd_nums.insert(integer_map[usize::try_from(int_pos_map[ii][jj]).unwrap()]);
                    }
                }
            }

            /*
            for num in nbd_nums.iter() {
                print!("{} ", num);
            }
            println!("");*/

            let gr_ratio = match nbd_nums.len() {
                2 => nbd_nums.into_iter().reduce(|prod, e| prod * e).unwrap(),
                _ => 0
            };

            ans += gr_ratio;
            j += 1;
        }
    }

    println!("{}", ans);

    Ok(())
}
