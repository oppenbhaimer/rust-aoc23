use std::fs::File;
use std::io::{self, BufRead};

const M3: &[(&str, char); 3] = &[ 
    ("one", '1'), 
    ("two", '2'), 
    ("six", '6'), 
];

const M4: &[(&str, char); 3] = &[
    ("four", '4'), 
    ("five", '5'), 
    ("nine", '9')
];

const M5: &[(&str, char); 3] = &[
    ("seven", '7'),
    ("eight", '8'),
    ("three", '3')
];

const M: &[&[(&str, char); 3]] = &[&M3, &M4, &M5];

fn main() -> io::Result<()> {
    let file = File::open("input/input_1")?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let l = format!("{}--", l);
        let mut digits: Vec<char> = vec![];

        let mut i = 0;
        for c in l.chars() {

            for j in [0, 1, 2] {
                if i >= j+2 {
                    let s3 = &l[i-j-2..i+1];
                    for (w, n) in M[j] {
                        if s3 == *w {
                            digits.push(*n);
                        }
                    }
                }
            }

            if c.is_digit(10) {
                digits.push(c);
            }

            i += 1;
        }

        for d in digits.iter() {
            print!("{} ", d);
        }
        println!("");

        let mut s = String::new();
        s.push(*digits.first().unwrap());
        s.push(*digits.last().unwrap());
        let n = s.parse::<i32>().unwrap();

        ans += n;
    }

    println!("{}", ans);

    Ok(())
}
