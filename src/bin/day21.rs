use std::collections::VecDeque;
use std::mem::ManuallyDrop;
use std::fs::File;
use regex::Regex;
use regex::Captures;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Eq, PartialEq)]
enum ModType {
    FlipFlop,
    Conjunction,
    Broadcast
}

impl From<&str> for ModType {
    fn from(a: &str) -> ModType {
        match a {
            "%" => ModType::FlipFlop,
            "&" => ModType::Conjunction,
            ""  => ModType::Broadcast,
            _   => ModType::Broadcast
        }
    }
}

// ideally be a union, but using a union gives me an unsafe error
struct ModState {
    conj: HashMap<String, bool>,
    ff: (bool, bool),
    bcast: i32
}

struct Module {
    typ: ModType,
    name: String,
    state: ModState,
    outputs: Vec<String>
}

impl From<&Captures<'_>> for Module {
    fn from(cap: &Captures) -> Module {
        let modtype = ModType::from(&cap[1]);
        let modname = cap[2].to_string();
        let modstate = match modtype {
            ModType::FlipFlop => ModState { conj: HashMap::new(), ff: (false, false), bcast: 0 },
            ModType::Conjunction => ModState { conj: HashMap::new(), ff: (false, false), bcast: 0 },
            ModType::Broadcast => ModState { conj: HashMap::new(), ff: (false, false), bcast: 0 }
        };
        let modoutputs : Vec<String> = cap[3].split(", ").map(|x| x.to_string()).collect();
        Module {
            typ: modtype,
            name: modname,
            state: modstate,
            outputs: modoutputs
        }
    }
}

impl Module {
    fn output(&self) -> Option<bool> {
        match self.typ {
            ModType::FlipFlop => {
                if self.state.ff.0 { return None; }
                else { return Some(self.state.ff.1); }
            },
            ModType::Conjunction => {
                let mut all_high = true;
                for (_, b) in self.state.conj.iter() {
                    if !b {
                        all_high = false;
                        break;
                    }
                }
                return Some(!all_high);
            },
            ModType::Broadcast => return Some(false)
        }
    }

    fn update_ff(&mut self, pulse: bool) {
        if !pulse {
            self.state.ff = (pulse, !self.state.ff.1);
        }
        else {
            self.state.ff = (pulse, self.state.ff.1);
        }
    }

    fn update_conj(&mut self, inp: &str, pulse: bool) {
        self.state.conj.insert(inp.to_string(), pulse);
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_20")?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"([%&]?)([a-z]+) -> ([a-z, ]+)").unwrap();

    let ops : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let ops : Vec<Captures> = ops.iter().map(|x| re.captures(x).unwrap()).collect();

    let mut mods : HashMap<String, Module> = HashMap::new();

    for op in ops {
        mods.insert(op[2].to_string(), Module::from(&op));
    }

    let mut conj_inputs : HashMap<String, Vec<String>> = HashMap::new();
    for (mname, module) in mods.iter() {
        for m in module.outputs.iter() {
            if mods.contains_key(m) && mods[m].typ == ModType::Conjunction {
                match conj_inputs.get_mut(m) {
                    Some(h) => { h.push(mname.clone()); },
                    None => { conj_inputs.insert(m.clone(), vec![mname.clone()]); }
                }
            }
        }
    }

    for (m, v) in conj_inputs {
        for u in v {
            mods.get_mut(&m).unwrap().state.conj.insert(u, false);
        }
    }

    let mut low_pulses : i64 = 0;
    let mut high_pulses : i64 = 0;

    for i in 0..10000 {
        let mut s : VecDeque<(String,Option<bool>,String)> = VecDeque::new();
        s.push_back(("button".to_string(), Some(false), "broadcaster".to_string()));
        while !s.is_empty() {
            let (src, sig, tgt) = s.pop_front().unwrap();
            if (src == "cx" || src == "gm" || src == "bf" || src == "qr") && sig == Some(false) {
                println!("{} -> {}", src, i);
            }
            match sig {
                Some(b) => {
                    if b { high_pulses += 1; }
                    else { low_pulses += 1; }
                    // println!("{} -{}-> {}", src, b, tgt);
                    if !mods.contains_key(&tgt) {
                        continue;
                    }
                    match mods[&tgt].typ {
                        ModType::Conjunction => mods.get_mut(&tgt).unwrap().update_conj(&src, b),
                        ModType::FlipFlop => mods.get_mut(&tgt).unwrap().update_ff(b),
                        ModType::Broadcast => {}
                    }
                    let o = mods[&tgt].output();
                    for t in mods[&tgt].outputs.iter() {
                        s.push_back((tgt.clone(), o, t.to_string()));
                    }
                },
                None => {}
            }
        }

        // let op0 = mods["gm"].output().unwrap();
        // let op1 = mods["bf"].output().unwrap();
        // let op2 = mods["cx"].output().unwrap();
        // let op3 = mods["qr"].output().unwrap();
        // println!("{}", op2);
        // println!("{} {} {} {}", op0, op1, op2, op3);
    }

    // println!("{} * {} = {}", low_pulses, high_pulses, low_pulses * high_pulses);

    Ok(())
}
