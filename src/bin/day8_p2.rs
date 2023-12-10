use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    }
    else {
        gcd(b, a%b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {

    return (a*b)/gcd(a, b);
}

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

    let mut start_nodes : Vec<String> = vec![];

    for key in tree.keys() {
        if key.ends_with("A") {
            start_nodes.push(key.to_string());
        }
    }

    let mut node_end_times : Vec<i64> = vec![];

    for node in start_nodes {

        let mut curr_node = node.as_str();
        let mut steps = 0;
        let mut dir_idx = 0;

        while !curr_node.ends_with("Z") {
            if directions[dir_idx] == 'L' {
                curr_node = &tree[curr_node].0;
            }
            else {
                curr_node = &tree[curr_node].1;
            }
            steps += 1;
            dir_idx = (dir_idx + 1) % directions.len();
        }

        node_end_times.push(steps); 
    }

    let mut ans = lcm(node_end_times[0], node_end_times[1]);

    for end_time in node_end_times.iter().skip(2) {
        ans = lcm(ans, *end_time);

    }

    println!("{}", ans);

    Ok(())
}
