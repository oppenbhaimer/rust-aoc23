use std::fs::File;
use std::io::{self, BufRead};

fn hash_fn(s: &str) -> i32 {
    // println!("{}", s);
    let mut ctr = 0;
    for c in s.chars() {
        ctr += c as i32;
        ctr *= 17;
        ctr %= 256;
    }
    ctr
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_15")?;
    let mut reader = io::BufReader::new(file);

    let mut line: String = String::new();
    let _ = reader.read_line(&mut line);
    let sequences: Vec<&str> = line.trim().split(",").collect();
    let mut ans: i32 = 0;

    for x in sequences {
        ans += hash_fn(x);
        println!("{} {}", x, hash_fn(x));
    }

    println!("{}", ans);

    Ok(())
}
