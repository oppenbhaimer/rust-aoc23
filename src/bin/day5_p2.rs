use std::fs::File;
use std::io::{self, BufRead};

// A range is [l, r) - integers in [l, r).
// A map is (l, r, s) - subtract s from all elements in [l, r)

/*          1 (-1)     2 (+2)
 *     ,----^---. ,----^---.
 *  O----O   O-----O   O----------O
 *
 *              |
 *              V 
 *
 *          1 (-1)     2 (+2)
 *     ,----^---. ,----^---.
 *  O--O-O   O--O-OO   O---O------O
 *
 *              |
 *              V 
 *
 *  O-OO-O   O--OOOO   O
 *
 *  O--O-O   O--O-OO   O---O------O
 *   0  1     1  0 2     2     0
 *
 *              |
 *              V 
 *
 *  O-O         OO          O-----O
 *    O-O   O--O    OO   O---O
 *
 *              |
 *              V 
 *
 *  O---O   O----O  OO   O--------O 
 *
 *          and so on...
 */
fn map_ranges(ranges: &Vec<(i64, i64)>, maps: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64)> {

    let mut temp_ranges : Vec<Vec<i64>> = vec![];

    for (l, r) in ranges.iter() {
        temp_ranges.push(vec![*l, *r]);
    }

    for (l, r, _sub) in maps.iter() {
        let mut k = 0;
        while k < temp_ranges.len() {
            let range_vec = &mut temp_ranges[k];
            let mut j = 0;
            while j < range_vec.len() - 1 {
                if range_vec[j] < *l && range_vec[j+1] > *l {
                    range_vec.insert(j+1, *l);
                }
                if range_vec[j] < *r && range_vec[j+1] > *r {
                    range_vec.insert(j+1, *r);
                }
                j += 1;
            }
            k += 1;
        }
    }

    let mut new_ranges : Vec<(i64, i64)> = vec![];

    for range_vec in temp_ranges.iter() {
        let mut j = 0;
        while j < range_vec.len() - 1 {
            new_ranges.push((range_vec[j], range_vec[j+1]));
            j += 1;
        }
    }

    // transform new_ranges 

    let mut transformed_ranges : Vec<(i64, i64)> = vec![];

    for (lr, rr) in new_ranges {
        let mut transformed = false;
        for (l, r, sub) in maps {
            if lr >= *l && lr < *r && rr > *l && rr <= *r {
                transformed_ranges.push((lr-sub, rr-sub));
                transformed = true;
                break;
            }
        }

        if !transformed {
            transformed_ranges.push((lr, rr));
        }
    }

    merge_common_intervals(&mut transformed_ranges)
}

// merges all intervals such that there are no overlapping intervals
fn merge_common_intervals(intervals: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {

    let mut final_intervals: Vec<(i64, i64)> = vec![];
    intervals.sort_by_key(|a| a.0);

    let mut i = 0;
    while i < intervals.len() {
        let interval = intervals[i];
        let final_l = interval.0;
        let mut final_r = interval.1;
        let j = i+1;
        while j < intervals.len() && intervals[j].0 < interval.1 {
            final_r = intervals[j].1;
        }

        final_intervals.push((final_l, final_r));

        i = j;

    }

    final_intervals
}

fn print_intervals(vec: &Vec<(i64, i64)>) {
    for (l, r) in vec {
        print!("({}, {}) ", l, r);
    }
    println!("");
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_5")?;
    let reader = io::BufReader::new(file);

    let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut seeds: Vec<(i64, i64)> = vec![];

    let mut i = 0;
    while i < lines.len() {
        let l = &lines[i];
        if l.starts_with("seeds: ") {
            let temp_seeds : Vec<_> = l[7 ..].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            let mut i = 0;
            while i < temp_seeds.len() {
                seeds.push((temp_seeds[i], temp_seeds[i]+temp_seeds[i+1]));
                i += 2;
            }
            seeds = merge_common_intervals(&mut seeds);
            print_intervals(&seeds);
        }
        else if l.contains("map:") {
            println!("{}", l);
            let mut map : Vec<(i64, i64, i64)> = vec![];
            i += 1;
            while i < lines.len() && lines[i] != "" {
                let values : Vec<i64> = lines[i].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
                map.push((values[1], values[1]+values[2], values[1]-values[0]));
                i += 1;
            }

            seeds = map_ranges(&mut seeds, &map);
            print_intervals(&seeds);
        }
        i += 1;
    }

    println!("{}", seeds[0].0);

    Ok(())
}
