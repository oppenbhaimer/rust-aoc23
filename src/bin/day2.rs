use std::fs::File;
use std::io::{self, BufRead};

/* p1
const R: i32 = 12;
const G: i32 = 13;
const B: i32 = 14;
*/

fn main() -> io::Result<()> {
    let file = File::open("input/input_2")?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let (game, history) = l.split_once(":").unwrap();
        let (_game, game_id) = game.trim().split_once(" ").unwrap();
        let game_id = game_id.parse::<i32>().unwrap();
        let history = history.split(";");

        let mut psum : i64 = 0;
        let mut mr : i64 = 0;
        let mut mg : i64 = 0;
        let mut mb : i64 = 0;

        for sample in history {
            let sample = sample.trim();
            let sample = sample.split(",");
            let mut r : i64 = 0;
            let mut g : i64 = 0;
            let mut b : i64 = 0;
            for datapt in sample {
                let (num, color) = datapt.trim().split_once(" ").unwrap();
                let num: i64 = num.parse().unwrap();
                match color {
                    "red" => r = num,
                    "green" => g = num,
                    "blue" => b = num,
                    _ => println!("Error: Format doesn't match")
                }
            }

            /* p1
            if r > R || g > G || b > B {
                possible = false;
                break;
            }
            */
            if r > mr { mr = r; }
            if g > mg { mg = g; }
            if b > mb { mb = b; }
        }

        psum += mr*mg*mb;
        ans += psum;

        /* p1
        if possible {
            ans += game_id;
        }
        */
    }

    println!("{}", ans);

    Ok(())
}
