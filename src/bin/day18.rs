use std::fs::File;
use regex::Regex;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_18")?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"([LRDU]) ([0-9]+) \(#([a-f0-9]+)\)").unwrap();

    let ops : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let ops : Vec<[&str; 3]> = ops.iter().map(|x| re.captures(x).unwrap().extract().1).collect();
    let ops : Vec<(&str, i64, i64)> = ops.iter().map(|x| (x[0], x[1].parse::<i64>().unwrap(), i64::from_str_radix(x[2], 16).unwrap())).collect();
    let mut ans : i64 = 0;

    let mut curr_pos : (i64, i64) = (0, 0);
    let mut shoelace_pts = vec![curr_pos];
    let mut loop_len : i64 = 0;

    for (dir, num, _hex) in ops {

        loop_len += num;
        curr_pos = match dir {
            "R" => (curr_pos.0+num, curr_pos.1),
            "U" => (curr_pos.0, curr_pos.1+num),
            "D" => (curr_pos.0, curr_pos.1-num),
            "L" => (curr_pos.0-num, curr_pos.1),
            _   => curr_pos
        };
        shoelace_pts.push(curr_pos);
    }
    shoelace_pts.push((0,0));

    for i in 1..shoelace_pts.len() {
        ans += (shoelace_pts[i-1].0 * shoelace_pts[i].1) as i64 - 
                (shoelace_pts[i-1].1 * shoelace_pts[i].0) as i64 ;
    }

    println!("{}", loop_len + ans.abs()/2 - loop_len/2 + 1); // pick thm

    Ok(())
}
