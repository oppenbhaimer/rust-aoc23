use std::fs::File;
use std::io::{self, BufRead};

fn map_nums(nums: &mut Vec<i64>, ranges: &Vec<(i64, i64, i64)>) {

    // naive O(n^2), can do better
    for num in nums.into_iter() {
        for range in ranges {
            if *num >= range.1 && *num <= range.1 + range.2 {
                *num = range.0 + (*num-range.1);
                break;
            }
        }
    }
}

fn print_vec<T:std::fmt::Display>(vec: &Vec<T>) {
    for v in vec {
        print!("{} ", v);
    }
    println!("");
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_5")?;
    let reader = io::BufReader::new(file);

    let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut seeds: Vec<i64> = vec![];

    let mut i = 0;
    while i < lines.len() {
        let l = &lines[i];
        if l.starts_with("seeds: ") {
            seeds = l[7 ..].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            print_vec(&seeds);
        }
        else if l.contains("map:") {
            println!("{}", l);
            let mut map : Vec<(i64, i64, i64)> = vec![];
            i += 1;
            while i < lines.len() && lines[i] != "" {
                let values : Vec<i64> = lines[i].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
                map.push((values[0], values[1], values[2]));
                i += 1;
            }

            map_nums(&mut seeds, &map);
            print_vec(&seeds);
        }
        i += 1;
    }

    println!("{}", seeds.iter().min().unwrap());

    Ok(())
}
