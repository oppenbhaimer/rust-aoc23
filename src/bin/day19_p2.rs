use std::fs::File;
use regex::Regex;
use regex::Captures;
use std::collections::BTreeMap;
use std::io::{self, BufRead};

enum Cmp {
    Gt,
    Lt,
    Unk
}

impl Cmp {
    fn compare(&self, qty: i32, other: i32) -> bool {
        match self {
            Cmp::Gt => other > qty,
            Cmp::Lt => other < qty,
            Cmp::Unk => true
        }
    }
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
    val: i32,
    tgt: String
}

impl Rule {
    fn accept(&self, p: &Part) -> bool {
        if self.cat == Category::U {
            return true;
        }
        for (cat, val) in p.values.iter() {
            if *cat == self.cat && self.cmp.compare(self.val, *val) {
                return true;
            }
        }
        false
    }
}
    
struct Part {
    values: Vec<(Category, i32)>
}

impl Part {
    fn value(&self) -> i32 {
        self.values.iter().fold(0, |acc, e| acc + e.1)
    }
}


impl From<&str> for Part {
    fn from(line: &str) -> Part {
        let re_part : Regex = Regex::new(r"([xmas])=([0-9]+)").unwrap();

        let n = line.len();
        let values : Vec<&str> = line[1..n-1].split(",").collect();
        let mut p = Part { values : vec![] };

        for value in values {
            let cap = re_part.captures(value).unwrap();
            p.values.push((Category::from(&cap[1]), cap[2].parse::<i32>().unwrap()));
        }
        p
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback: String

}

impl Workflow {
    fn process(&self, p: &Part) -> String {
        for rule in self.rules.iter() {
            if rule.accept(p) {
                return rule.tgt.clone();
            }
        }
        return self.fallback.clone();
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
                val: rule[3].parse::<i32>().unwrap(),
                tgt: rule[4].to_string()
            });
        }
        wf
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_19")?;
    let reader = io::BufReader::new(file);

    let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let mut workflows: BTreeMap<String, Workflow> = BTreeMap::new();
    let mut ans : i32 = 0;

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

    let mut parts: Vec<Part> = vec![];
    let mut l = it.next();
    while l != None {
        let line = l.unwrap();
        let part = Part::from(line.as_str());
        parts.push(part);
        l = it.next();
    }

    for part in parts {
        let mut curr_wf = "in".to_string();
        while curr_wf != "A" && curr_wf != "R" {
            curr_wf = workflows[&curr_wf].process(&part);
            print!("{} -> ", curr_wf);
        }
        println!("");

        if curr_wf == "A" {
            ans += part.value();
        }
    }

    println!("{}", ans);

    Ok(())
}
