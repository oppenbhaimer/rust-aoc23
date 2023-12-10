use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_8")?;
    let reader = io::BufReader::new(file);

    let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let directions : Vec<char> = lines[0].chars().collect();
    let nodes = &lines[2..];

    let mut tree : HashMap<String, (String,String)> = HashMap::new();
    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    for node in nodes {
        let (_, [name, left, right]) = re.captures(node).unwrap().extract();
        tree.insert(name.to_string(), (left.to_string(), right.to_string()));
    }

    let mut curr_node = "AAA";
    let mut ans = 0;
    let mut dir_idx = 0;

    while curr_node != "ZZZ" {
        if directions[dir_idx] == 'L' {
            curr_node = &tree[curr_node].0;
        }
        else {
            curr_node = &tree[curr_node].1;
        }
        ans += 1;
        dir_idx = (dir_idx + 1) % directions.len();
    }

    println!("{}", ans);

    Ok(())
}
