use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    htype: HandType,
    hand: (i32, i32, i32, i32, i32),
    hand_str: String,
    score: i32
}

impl Hand {

    fn new(hand: &str, score: i32) -> Hand {
        let htype = get_type(hand);
        let mut t: Vec<i32> = vec![2; 5];

        for (i, ch) in hand.chars().enumerate() {
            t[i] = match ch {
                '2' ..= '9' => ch.to_digit(10).unwrap().try_into().unwrap(),
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _   => 0
                }
        };
        
        Hand {
            htype: htype,
            hand: (t[0], t[1], t[2], t[3], t[4]),
            hand_str: hand.to_string(),
            score: score
        }
    }
}

impl PartialEq for Hand {

    fn eq(&self, other: &Self) -> bool {
        (self.htype, self.hand) == (other.htype, other.hand)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

impl Ord for Hand {

    fn cmp(&self, other: &Self) -> Ordering {
        (self.htype, self.hand).cmp(&(other.htype, other.hand))
    }
}


fn get_type(hand: &str) -> HandType {
    let mut freq_map: HashMap<char,i32> = HashMap::new();
    for ch in hand.chars() {
        match freq_map.get_mut(&ch) {
            Some(a) => {
                *a += 1;
            },
            None => {
                freq_map.insert(ch, 1);
            },
        }
    }

    if freq_map.len() == 1 {
        HandType::FiveOfAKind
    }
    else if freq_map.len() == 2 {
        let (a, b) = freq_map.into_iter().next().unwrap();
        if b == 4 || b == 1 {
            HandType::FourOfAKind
        }
        else {
            HandType::FullHouse
        }
    }
    else if freq_map.len() == 3 {
        let (a1, b1) = freq_map.iter().next().unwrap();
        let (a2, b2) = freq_map.iter().skip(1).next().unwrap();
        let (a3, b3) = freq_map.iter().skip(2).next().unwrap();

        if (*b1 == 2 && *b2 == 2) || (*b1 == 2 && *b2 == 1) || (*b1 == 1 && *b2 == 2) {
            HandType::TwoPair
        }
        else {
            HandType::ThreeOfAKind
        }

    }
    else if freq_map.len() == 5 {
        HandType::HighCard
    }
    else {
        HandType::OnePair
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_7")?;
    let reader = io::BufReader::new(file);

    let lines : Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let hands : Vec<(&str,&str)> = lines.iter().map(|x| x.split_once(" ").unwrap()).collect();
    let mut hands : Vec<Hand> = hands.into_iter().map(|x| Hand::new(x.0, x.1.parse::<i32>().unwrap())).collect();

    hands.sort();

    let mut ans: usize = 0;

    for (i, h) in hands.iter().enumerate() {
        // println!("{}", h.hand_str);
        ans += (i+1)*(h.score as usize);

    }

    println!("{}", ans);

    Ok(())
}
