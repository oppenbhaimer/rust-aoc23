use std::fs::File;
use pathfinding::directed::dijkstra::*;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Up,
    Down,
    Left
}

use Direction::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Node {
    pos: (usize, usize),
    dir: Direction,
    steps: usize
}

fn main() -> io::Result<()> {
    let file = File::open("input/input_17")?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<i32>> = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|y| (y as i32) - ('0' as i32))
                .collect()
        })
        .collect();
    let n : usize = grid.len(); // y bounds
    let m : usize = grid[0].len(); // x bounds

    let mut g: HashMap<Node, Vec<(Node, i32)>> = HashMap::new();

    for y in 0..n {
        for x in 0..m {
            for dir in [Down, Up, Right, Left] {
                for s in 4..=10 {
                    let node = Node{pos: (x, y), dir: dir.clone(), steps: s};
                    let mut nbd: Vec<(Node, i32)> = vec![];

                    match dir {
                        Down => {
                            if y < n-1 && s != 10 {
                                nbd.push((Node{pos: (x, y+1), dir: Down, steps:s+1}, grid[y+1][x]));
                            }
                            if x < m-4 {
                                nbd.push((Node{pos: (x+4, y), dir: Right, steps: 4}, grid[y][x+1]+grid[y][x+2]+grid[y][x+3]+grid[y][x+4]));
                            }
                            if x > 3 {
                                nbd.push((Node{pos: (x-4, y), dir: Left, steps: 4}, grid[y][x-1]+grid[y][x-2]+grid[y][x-3]+grid[y][x-4]));
                            }
                        },
                        Up => {
                            if y > 0 && s != 10 {
                                nbd.push((Node{pos: (x, y-1), dir: Up, steps: s+1}, grid[y-1][x]));
                            }
                            if x < m-4 {
                                nbd.push((Node{pos: (x+4, y), dir: Right, steps: 4}, grid[y][x+1]+grid[y][x+2]+grid[y][x+3]+grid[y][x+4]));
                            }
                            if x > 3 {
                                nbd.push((Node{pos: (x-4, y), dir: Left, steps: 4}, grid[y][x-1]+grid[y][x-2]+grid[y][x-3]+grid[y][x-4]));
                            }
                        },
                        Right => {
                            if x < m-1 && s != 10 {
                                nbd.push((Node{pos: (x+1, y), dir: Right, steps: s+1}, grid[y][x+1]));
                            }
                            if y > 3 {
                                nbd.push((Node{pos: (x, y-4), dir: Up, steps: 4}, grid[y-1][x]+grid[y-2][x]+grid[y-3][x]+grid[y-4][x]));
                            }
                            if y < n-4 {
                                nbd.push((Node{pos: (x, y+4), dir: Down, steps: 4}, grid[y+1][x]+grid[y+2][x]+grid[y+3][x]+grid[y+4][x]));
                            }
                        },
                        Left => {
                            if x > 0 && s != 10 {
                                nbd.push((Node{pos: (x-1, y), dir: Left, steps: s+1}, grid[y][x-1]));
                            }
                            if y > 3 {
                                nbd.push((Node{pos: (x, y-4), dir: Up, steps: 4}, grid[y-1][x]+grid[y-2][x]+grid[y-3][x]+grid[y-4][x]));
                            }
                            if y < n-4 {
                                nbd.push((Node{pos: (x, y+4), dir: Down, steps: 4}, grid[y+1][x]+grid[y+2][x]+grid[y+3][x]+grid[y+4][x]));
                            }
                        }
                    }

                    g.insert(node, nbd);
                }
            }
        }
    }

    // start node
    g.insert(Node{pos: (0, 0), dir: Right, steps: 0}, vec![
             (Node{pos: (0, 4), dir: Down, steps: 4}, grid[1][0]+grid[2][0]+grid[3][0]+grid[4][0]),
             (Node{pos: (4, 0), dir: Right, steps: 4}, grid[0][1]+grid[0][2]+grid[0][3]+grid[0][4])
            ]);

    // dijkstra on g

    let (path, dist) = dijkstra(&Node{pos: (0, 0), dir: Right, steps: 0}, |p| g.get(p).unwrap().clone(), |p| p.pos == (m-1, n-1)).unwrap();
    // let dists = dijkstra(&g, Node{pos: (0, 0), dir: Right, steps: 1});
    //
    // let hmap = dijkstra_all(&Node{pos: (0, 0), dir: Right, steps: 0}, |p| g.get(p).unwrap().clone());
    // for (node, (n, c)) in hmap {
    //     println!("{} {}", node.pos.0, node.pos.1);
    // }
    // for node in path {
    //     println!("{} {}", node.pos.0, node.pos.1);
    // }


    println!("{}", dist);

    Ok(())
}
