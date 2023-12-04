use std::fs::File;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_4")?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let mut dp: Vec<i64> = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let (_card_info, card_nums) = line.split_once(":").unwrap();

        let (winning_nums, our_nums) = card_nums.trim().split_once(" | ").unwrap();
        let winning_nums : HashSet<i32> = winning_nums.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let our_nums : Vec<i32> = our_nums.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let mut matches : i64 = 0;
        for num in our_nums {
            if winning_nums.contains(&num) {
                matches += 1;
            }
        }

        // println!("{} - {} ({})", i+1, matches, dp[i]);

        for idx in i+1 ..= i+usize::try_from(matches).unwrap() {
            if idx < lines.len() {
                dp[idx] += dp[i];
            }
            else {
                break;
            }
        }

        ans += dp[i];
    }

    println!("{}", ans);

    Ok(())
}
