use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input/input_9")?;
    let reader = io::BufReader::new(file);

    let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let sequences : Vec<Vec<i64>> = lines.iter().map(
        |x| x.split(" ").map(
            |y| y.parse::<i64>().unwrap()).collect()
        ).collect();

    let mut ans : i64 = 0;
    for seq in sequences {
        let mut diffs : Vec<Vec<i64>> = vec![seq];
        // zero check not required at the first stage, checked input.
        let mut all_z = false;
        let mut j = 0;

        while !all_z {
            all_z = true;
            let mut v : Vec<i64> = vec![];
            for i in 0..diffs[j].len()-1 {
                let val = diffs[j][i+1] - diffs[j][i];
                v.push(val);
                all_z = all_z && (val == 0);
            }
            diffs.push(v);
            j += 1;
        }

        let last_idx = diffs.len() - 1;
        diffs[last_idx].push(0);

        /* p1
        for k in (0..diffs.len()-1).rev() {
            let l1 = *diffs[k+1].last().unwrap();
            let l2 = *diffs[k].last().unwrap();
            diffs[k].push(l1+l2);
        }*/

        // p2
        for k in (0..diffs.len()-1).rev() {
            let l1 = *diffs[k+1].last().unwrap();
            let l2 = *diffs[k].first().unwrap();
            diffs[k].push(l2-l1);
        }

        /*
        for diff in &diffs {
            for i in diff {
                print!("{} ", i);
            }
            println!("");
        }*/

        ans += *diffs[0].last().unwrap();
    }

    println!("{}", ans);

    Ok(())
}
