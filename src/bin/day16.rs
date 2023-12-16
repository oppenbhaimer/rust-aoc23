use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Cell {
    Reflect45,
    Reflect135,
    SplitVertical,
    SplitHorizontal,
    Empty,
    Unknown,
}

impl From<char> for Cell {
    fn from(ch: char) -> Cell {
        match ch {
            '.' => Cell::Empty,
            '/' => Cell::Reflect45,
            '\\' => Cell::Reflect135,
            '|' => Cell::SplitVertical,
            '-' => Cell::SplitHorizontal,
            _ => Cell::Unknown,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

use Cell::*;
use Direction::*;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Beam {
    pos: (i32, i32),
    dir: Direction,
}

fn transform(b: Beam, d: Direction) -> Beam {
    match d {
        Up => Beam {
            pos: (b.pos.0, b.pos.1 - 1),
            dir: Up,
        },
        Down => Beam {
            pos: (b.pos.0, b.pos.1 + 1),
            dir: Down,
        },
        Right => Beam {
            pos: (b.pos.0 + 1, b.pos.1),
            dir: Right,
        },
        Left => Beam {
            pos: (b.pos.0 - 1, b.pos.1),
            dir: Left,
        },
    }
}

fn operate(c: Cell, b: Beam) -> [Option<Beam>; 2] {
    match c {
        Empty => [Some(transform(b, b.dir)), None],
        Reflect45 => match b.dir {
            Right => [Some(transform(b, Up)), None],
            Left => [Some(transform(b, Down)), None],
            Up => [Some(transform(b, Right)), None],
            Down => [Some(transform(b, Left)), None],
        },
        Reflect135 => match b.dir {
            Right => [Some(transform(b, Down)), None],
            Left => [Some(transform(b, Up)), None],
            Up => [Some(transform(b, Left)), None],
            Down => [Some(transform(b, Right)), None],
        },
        SplitVertical => match b.dir {
            Right => [Some(transform(b, Up)), Some(transform(b, Down))],
            Left => [Some(transform(b, Down)), Some(transform(b, Up))],
            _ => [Some(transform(b, b.dir)), None],
        },
        SplitHorizontal => match b.dir {
            Up => [Some(transform(b, Right)), Some(transform(b, Left))],
            Down => [Some(transform(b, Right)), Some(transform(b, Left))],
            _ => [Some(transform(b, b.dir)), None],
        },
        Unknown => [None, None],
    }
}

impl std::fmt::Display for Beam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.dir {
            Up => write!(f, "({},{}), ^", self.pos.0, self.pos.1),
            Down => write!(f, "({},{}), V", self.pos.0, self.pos.1),
            Left => write!(f, "({},{}), <", self.pos.0, self.pos.1),
            Right => write!(f, "({},{}), >", self.pos.0, self.pos.1),
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_16")?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<Cell>> = reader
        .lines()
        .map(|x| x.unwrap().chars().map(|y| Cell::from(y)).collect())
        .collect();
    let n = grid.len();
    let m = grid[0].len();

    // dfs the beam
    let mut traversed: HashSet<Beam> = HashSet::new();

    let mut stack: Vec<Beam> = vec![];
    stack.push(Beam {
        pos: (0, 0),
        dir: Right,
    });

    while !stack.is_empty() {
        let b = stack.pop().unwrap();
        // println!("Popping {}", b);
        let (x, y) = b.pos;
        if x < 0 || x >= n as i32 || y < 0 || y >= m as i32 || traversed.contains(&b) {
            continue;
        }
        traversed.insert(b);
        let cells = operate(grid[y as usize][x as usize], b);

        for cell in cells {
            match cell {
                Some(p) => {
                    // println!("Pushing {}", p);
                    stack.push(p);
                }
                None => {}
            }
        }
    }

    let mut energized: Vec<Vec<bool>> = vec![vec![false; m]; n];

    for b in traversed {
        energized[b.pos.1 as usize][b.pos.0 as usize] = true;
    }

    let ans = energized.iter().fold(0, |acc, x| {
        acc + x.iter().fold(0, |bcc, y| bcc + (*y as i32))
    });

    println!("{}", ans);

    Ok(())
}
