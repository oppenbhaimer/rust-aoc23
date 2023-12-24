use std::fs::File;
use regex::Regex;
use regex::Captures;
use std::collections::{BTreeMap, HashMap};
use std::io::{self, BufRead};

type Vec4 = [i64; 4];

enum Cmp {
    Gt,
    Lt,
    Unk
}

impl From<&str> for Cmp {
    fn from(s: &str) -> Cmp {
        match s {
            "<" => Cmp::Lt,
            ">" => Cmp::Gt,
            _   => Cmp::Unk,
        }
    }
}


#[derive(PartialEq, Eq)]
enum Category {
    X, M, A, S, U
}

impl From<&str> for Category {
    fn from(s: &str) -> Category {
        match s {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _   => Category::U,
        }
    }
}

struct Rule {
    cat: Category,
    cmp: Cmp,
    val: i64,
    tgt: String
}

impl Rule {
    fn split_vol(&self, s: Vec4, e: Vec4) -> (Option<(Vec4, Vec4)>, Option<(Vec4, Vec4)>) {
        // (Yes, No)
        let mut i1 = None;
        let mut i2 = None;
        let idx = match self.cat {
            Category::X => 0,
            Category::M => 1,
            Category::A => 2,
            Category::S => 3,
            Category::U => 0
        };

        match self.cmp {
            Cmp::Gt => {
                if self.val >= e[idx] {
                    i1 = None;
                    i2 = Some((s, e));
                }
                else if self.val < s[idx] {
                    i1 = Some((s, e));
                    i2 = None;
                }
                else {
                    let mut m = e;
                    m[idx] = self.val;
                    let mut m2 = s;
                    m2[idx] = self.val+1;

                    i1 = Some((m2, e));
                    i2 = Some((s, m));
                }
            },
            Cmp::Lt => {
                if self.val > e[idx] {
                    i1 = Some((s, e));
                    i2 = None;
                }
                else if self.val <= s[idx] {
                    i1 = None;
                    i2 = Some((s, e));
                }
                else {
                    let mut m = e;
                    m[idx] = self.val-1;
                    let mut m2 = s;
                    m2[idx] = self.val;

                    i1 = Some((s, m));
                    i2 = Some((m2, e));
                }
            },
            Cmp::Unk => {}
        }

        return (i1, i2);
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback: String

}

impl Workflow {
    fn split(&self, start: Vec4, end: Vec4) -> Vec<(String, (Vec4, Vec4))> {
        let mut start = start;
        let mut end = end;
        let mut map = vec![];
        for rule in self.rules.iter() {
            let (i1, i2) = rule.split_vol(start, end);
            match i1 {
                Some(b) => {
                    // println!("{}: ({}, {}, {}, {}), ({}, {}, {}, {})", rule.tgt, b.0[0], b.0[1], b.0[2], b.0[3], b.1[0], b.1[1], b.1[2], b.1[3]);
                    map.push((rule.tgt.clone(), b));}
                None => {}
            }
            match i2 {
                Some((s, e)) => {
                    start = s;
                    end = e;
                },
                None => {break;}
            }
        }
        // println!("{}: ({}, {}, {}, {}), ({}, {}, {}, {})", self.fallback, start[0], start[1], start[2], start[3], end[0], end[1], end[2], end[3]);
        map.push((self.fallback.clone(), (start, end)));
        map
    }

}

impl From<&str> for Workflow {

    fn from(line: &str) -> Workflow {

        let re_workflow : Regex = Regex::new(r"([a-z]+)\{(.*)\}").unwrap();
        let re_rule : Regex = Regex::new(r"([xmas]+)(<|>)([0-9]+):([a-zAR]+)").unwrap();

        let (_, [name, rules]) = re_workflow.captures(&line).unwrap().extract();
        let rules : Vec<&str> = rules.split(",").collect();
        let mut wf = Workflow {
            name: name.to_string(),
            rules: vec![],
            fallback: rules.last().unwrap().to_string()
        };
        let rules : Vec<Captures> = rules.iter().take(rules.len()-1).map(|x| re_rule.captures(x).unwrap()).collect();
        for rule in rules {
            wf.rules.push(Rule{
                cat: Category::from(&rule[1]),
                cmp: Cmp::from(&rule[2]),
                val: rule[3].parse::<i64>().unwrap(),
                tgt: rule[4].to_string()
            });
        }
        wf
    }
}

fn vol(s: Vec4, e: Vec4) -> i64 {
    let mut prod = 1;
    for i in 0..4 {
        prod *= e[i]-s[i]+1;
    }
    prod
}

fn get_num_combs(start: Vec4, end: Vec4, wf: &Workflow, wfs: &BTreeMap<String, Workflow>) -> i64 {

    let mut sum = 0;

    let splits = wf.split(start, end);

    for (tgt, (s, e)) in splits {
        if tgt == "A" {
            // println!("Accepting ({}, {}, {}, {}), ({}, {}, {}, {})", s[0], s[1], s[2], s[3], e[0], e[1], e[2], e[3]);
            sum += vol(s, e);
        }
        else if tgt != "R" {
            // println!("{}", tgt);
            sum += get_num_combs(s, e, &wfs[&tgt], wfs);
        }
    }

    sum
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_19")?;
    let reader = io::BufReader::new(file);

    let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let mut workflows: BTreeMap<String, Workflow> = BTreeMap::new();

    let mut it = lines.iter();
    let mut l = it.next();

    while l != None {
        let line = l.unwrap();
        if line == "" {
            break;
        }
        let wf = Workflow::from(line.as_str());
        let wf_name = wf.name.clone();
        workflows.insert(wf_name, wf);
        l = it.next();
    }

    // recurse on parts

    println!("{}", get_num_combs([1,1,1,1], [4000,4000,4000,4000], &workflows["in"], &workflows));

    Ok(())
}
