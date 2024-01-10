use std::fs::File;
use regex::Regex;
use regex::Captures;
use std::io::{self, BufRead};

struct Vec3 {
    x: i32,
    y: i32,
    z: i32
}

struct Brick {
    start: Vec3,
    end: Vec3 
}

impl From<&Captures<'_>> for Brick {
    fn from(cap: &Captures) -> Brick {
        Brick {
            start: Vec3 {
                x: cap[1].parse::<i32>().unwrap(),
                y: cap[2].parse::<i32>().unwrap(),
                z: cap[3].parse::<i32>().unwrap()
            },
            end  : Vec3 {
                x: cap[4].parse::<i32>().unwrap(),
                y: cap[5].parse::<i32>().unwrap(),
                z: cap[6].parse::<i32>().unwrap()
            }
        }
    }
}


fn main() -> io::Result<()> {
    let file = File::open("test_22")?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();

    let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let caps : Vec<Captures> = lines.iter().map(|x| re.captures(x).unwrap()).collect();
    let bricks : Vec<Brick> = caps.iter().map(|x| Brick::from(x)).collect();

    let voxels: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; 10]; 10]; 500];

    let supporting: Vec<bool> = vec![false; bricks.len()];
    // make bricks fall until they collide. On collision, mark the supporting 
    // bricks as fixed
    // Collision check: need a Data Structure that sorts and maintains bricks by 
    // Z-position and also allows fast indexing by Z position.


    Ok(())
}
