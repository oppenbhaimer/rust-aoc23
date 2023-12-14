use std::fs::File;
use std::io::{self, BufRead};

fn prefix(s: &[char], n: i32) -> bool {
    let n = n as usize;
    let mut k = n < s.len() && (s[n] == '.' || s[n] == '?');

    if !k {
        return false;
    }

    for i in 0..n {
        if s[i] == '.' {
            return false;
        }
    }
    return true;
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_12")?;
    let reader = io::BufReader::new(file);

    let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let mut ans : i64 = 0;

    for l in lines {

        let (pattern, blocks) = l.split_once(" ").unwrap();
        let blocks : Vec<i32> = blocks.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        // p2
        let blocks = [&blocks[..], &blocks[..], &blocks[..], &blocks[..], &blocks[..]].concat();
        let mut pattern : Vec<char> = pattern.chars().collect();
        // p2
        pattern.push('?');
        let mut pattern : Vec<char> = [&pattern[..], &pattern[..], &pattern[..], &pattern[..], &pattern[..]].concat();
        let plen = pattern.len();
        pattern[plen-1] = '.';
        // pattern.push('.');


        let mut dp: Vec<Vec<i64>> = vec![vec![0; pattern.len() + 1]; blocks.len() + 1];
        dp[0][pattern.len()] = 1;
        for idx in (0..pattern.len()).rev() {
            if pattern[idx] == '#' {
                break;
            }
            dp[0][idx] = 1;
        }

        for i in 1..blocks.len()+1 {
            let n = blocks[blocks.len()-i];
            for j in (0..pattern.len()).rev() {
                if pattern[j] == '?' {
                    dp[i][j] += dp[i][j+1];
                    if prefix(&pattern[j..], n) { 
                        dp[i][j] += dp[i-1][j+(n as usize)+1];
                    }
                }
                else if pattern[j] == '.' {
                    dp[i][j] = dp[i][j+1];
                }
                else {
                    if prefix(&pattern[j..], n) { 
                        dp[i][j] += dp[i-1][j+(n as usize)+1];
                    }
                }
            }
        }

        /*
        for char in pattern {
            print!("{} ", char);
        }
        println!("");
        for row in dp.iter() {
            for col in row {
                print!("{} ", col);
            }
            println!("");
        }
        println!("{}", dp[blocks.len()][0]);
        */

        ans += dp[blocks.len()][0];
    }

    println!("{}", ans);

    Ok(())
}
