use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn hash_fn(s: &str) -> usize {
    // println!("{}", s);
    let mut ctr = 0;
    for c in s.chars() {
        ctr += c as usize;
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
    let mut boxes: Vec<Vec<(&str, i32)>> = vec![vec![]; 256];
    let mut ans: i64 = 0;

    let re = Regex::new(r"([a-z]+)(=|-)([0-9]*)").unwrap();
    for x in sequences {
        let (_, [id, op, pwr]) = re.captures(x).unwrap().extract();
        let i = hash_fn(id);

        if op == "-" {
            let index = boxes[i].iter().position(|x| x.0 == id);
            match index {
                Some(idx) => {
                    let _ = boxes[i].remove(idx);
                }
                None => {}
            }
        } else {
            let pwr = pwr.parse::<i32>().unwrap();
            let mut subst = false;
            for b in boxes[i].iter_mut() {
                if b.0 == id {
                    *b = (id, pwr);
                    subst = true;
                    break;
                }
            }

            if !subst {
                boxes[i].push((id, pwr));
            }
        }
    }

    for (i, lenses) in boxes.iter().enumerate() {
        for (j, (_, pwr)) in lenses.iter().enumerate() {
            ans += ((i + 1) * (j + 1) * (*pwr as usize)) as i64;
        }
    }

    println!("{}", ans);

    Ok(())
}
