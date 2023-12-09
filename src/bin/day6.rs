use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_6")?;
    let reader = io::BufReader::new(file);

    // comment out for part 2
    // let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    // let times : Vec<f64> = lines[0].split_whitespace().skip(1).map(|x| x.parse::<f64>().unwrap()).collect();
    // let distances : Vec<f64> = lines[1].split_whitespace().skip(1).map(|x| x.parse::<f64>().unwrap()).collect();

    // uncomment for part 2
    let times : Vec<f64> = vec![47847467.0];
    let distances : Vec<f64> = vec![207139412091014.0];

    let mut prod = 1;

    for (t, d) in times.iter().zip(distances.iter()) {

        // roots
        let mut r1 = (t - (t*t - 4.0*d).sqrt())/2.0;
        let mut r2 = (t + (t*t - 4.0*d).sqrt())/2.0;

        if r1.ceil() - r1 < 1e-6 {
            r1 += 1.0;
        }
        if r2 - r2.floor() < 1e-6 {
            r2 -= 1.0;
        }

        let n1 = r1.ceil() as i32;
        let n2 = r2.floor() as i32;

        println!("{} {}", n1, n2);

        prod *= n2-n1+1;
    }

    println!("{}", prod);

    Ok(())
}
