use std::fs::File;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_4")?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (_card_info, card_nums) = line.split_once(":").unwrap();

        let (winning_nums, our_nums) = card_nums.trim().split_once(" | ").unwrap();
        let winning_nums : HashSet<i32> = winning_nums.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let our_nums : Vec<i32> = our_nums.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let mut pts = 0;
        for num in our_nums {
            if winning_nums.contains(&num) {
                if pts == 0 {
                    pts = 1;
                }
                else {
                    pts *= 2;
                }
            }
        }

        ans += pts;
    }

    println!("{}", ans);

    Ok(())
}
