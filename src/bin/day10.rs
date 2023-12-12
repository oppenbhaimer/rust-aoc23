use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_10")?;
    let reader = io::BufReader::new(file);

    let grid : Vec<Vec<char>> = reader.lines().map(|x| x.unwrap().chars().collect()).collect();
    let mut ans : i32 = 0;

    let mut s_pos = (0, 0);
    let mut found = false;

    for (i, l) in grid.iter().enumerate() {
        if found {
            break;
        }
        for (j, c) in l.iter().enumerate() {
            if *c == 'S' {
                s_pos = (i, j);
                found = true;
                break;
            }
        }
    }

    let mut loop_len = 1;
    // picked this by looking at the map
    let mut curr_pos = (s_pos.0+1, s_pos.1);
    let mut curr_dir = "BOTTOM";

    let mut shoelace_pts = vec![s_pos];

    while curr_pos != s_pos {
        let curr_char = grid[curr_pos.0][curr_pos.1];

        if curr_char == 'F' {
            shoelace_pts.push(curr_pos);
            if curr_dir == "LEFT" {
                curr_pos = (curr_pos.0+1, curr_pos.1);
                curr_dir = "BOTTOM";
            }
            else if curr_dir == "TOP" {
                curr_pos = (curr_pos.0, curr_pos.1+1);
                curr_dir = "RIGHT";
            }
        }
        else if curr_char == '7' {
            shoelace_pts.push(curr_pos);
            if curr_dir == "RIGHT" {
                curr_pos = (curr_pos.0+1, curr_pos.1);
                curr_dir = "BOTTOM";
            }
            else if curr_dir == "TOP" {
                curr_pos = (curr_pos.0, curr_pos.1-1);
                curr_dir = "LEFT";
            }
        }
        else if curr_char == 'L' {
            shoelace_pts.push(curr_pos);
            if curr_dir == "LEFT" {
                curr_pos = (curr_pos.0-1, curr_pos.1);
                curr_dir = "TOP";
            }
            else if curr_dir == "BOTTOM" {
                curr_pos = (curr_pos.0, curr_pos.1+1);
                curr_dir = "RIGHT";
            }
        }
        else if curr_char == 'J' {
            shoelace_pts.push(curr_pos);
            if curr_dir == "RIGHT" {
                curr_pos = (curr_pos.0-1, curr_pos.1);
                curr_dir = "TOP";
            }
            else if curr_dir == "BOTTOM" {
                curr_pos = (curr_pos.0, curr_pos.1-1);
                curr_dir = "LEFT";
            }
        }
        else if curr_char == '|' {
            if curr_dir == "BOTTOM" {
                curr_pos = (curr_pos.0+1, curr_pos.1);
                curr_dir = "BOTTOM";
            }
            else if curr_dir == "TOP" {
                curr_pos = (curr_pos.0-1, curr_pos.1);
                curr_dir = "TOP";
            }
        }
        else if curr_char == '-' {
            if curr_dir == "RIGHT" {
                curr_pos = (curr_pos.0, curr_pos.1+1);
                curr_dir = "RIGHT";
            }
            else if curr_dir == "LEFT" {
                curr_pos = (curr_pos.0, curr_pos.1-1);
                curr_dir = "LEFT";
            }
        }

        loop_len += 1;
    }

    shoelace_pts.push(s_pos);

    // part 1
    // println!("{}", loop_len/2);
    
    // part 2

    for i in 1..shoelace_pts.len() {
        ans += (shoelace_pts[i-1].0 * shoelace_pts[i].1) as i32 - 
                (shoelace_pts[i-1].1 * shoelace_pts[i].0) as i32 ;
    }

    println!("{}", ans.abs()/2 - loop_len/2 + 1); // pick thm

    Ok(())
}
