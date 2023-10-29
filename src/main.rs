use std::mem::swap;

use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

use crate::Direction::{Add, Both, Nowhere, Sub};

const MAX_WEIGHT: usize = 10;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Add,
    Sub,
    Both,
    Nowhere,
}

impl Direction {
    pub fn from_bool(left: bool, right: bool) -> Self {
        if left && right {
            return Both;
        }
        if left {
            return Add;
        }
        if right {
            return Sub;
        }
        Nowhere
    }
}

fn main() {
    for _ in 0..1 {
        simulate();
        println!("\n#\n");
    }
}

fn simulate() {
    let N = 8;
    let mut g: Vec<usize> = thread_rng().sample_iter(Uniform::new(1, MAX_WEIGHT + 1)).take(N).collect();
    println!("weights: {:?}", g);

    let G: usize = g.iter().sum(); // G
    println!("sum weights: {}", G);

    let range_i = 1..=N;
    let range_x = 0..=G;

    let mut S = vec![vec![None; G + 1]; N + 1];

    for i in range_i.clone() {
        for x in range_x.clone() {
            // base cases
            if i == 1 {
                S[i][x] = Some((x == g[i - 1], Nowhere));
            } else {
                let gi = g[i - 1] as i64;

                let idx1 = (x as i64 + gi).abs() as usize;
                let idx2 = (x as i64 - gi).abs() as usize;

                let left = S[i - 1].get(idx1).is_some_and(|a| a.unwrap().0);
                let right = S[i - 1].get(idx2).is_some_and(|a| a.unwrap().0);

                let bag = Direction::from_bool(left, right);

                S[i][x] = Some((left || right, bag));

                if left || right {
                    println!("S({i:2}, {x:2}) = T ({gi:2}) ({idx1:2}, {idx2:2}) ({left:5}, {right:5}) -> {:?}", bag);
                }
            }
        }
    }

    let mut lowest_x = 0;

    for x in range_x.clone() {
        if let Some(b) = S[N][x] {
            if b.0 {
                println!("lowest = S({N}, {x}) = T");
                lowest_x = x as i64;
                break;
            }
        }
    }

    print_table(&g, &S, &Vec::new());

    // find bags
    let mut path = Vec::new();
    let mut left = Vec::new();
    let mut right = Vec::new();

    let mut current_x = lowest_x;
    for i in (1..=N).rev() {
        let current = S[i][current_x as usize].unwrap();
        path.push((i, current_x as usize));
        assert!(current.0);
        let gi = g[i - 1] as i64;
        match current.1 {
            Add | Both => {
                left.push(gi);
                let x = current_x;
                current_x = (current_x + gi).abs();
                println!("[{i:2}][{x:2}] adding {gi} to left, going right new x = {current_x}");
            }
            Sub | Nowhere => {
                right.push(gi);
                let x = current_x;
                current_x = (current_x - gi).abs();
                println!("[{i:2}][{x:2}] adding {gi} to right, going left new x = {current_x}");
                if x - gi < 0 {
                    println!("switching left and right");
                    swap(&mut left, &mut right);
                }
            }
        }
    }

    let ls: i64 = left.iter().sum();
    let rs: i64 = right.iter().sum();
    println!("{left:?} = {ls}");
    println!("{right:?} = {rs}");
    println!("abs({} - {}) = {}", ls, rs, (ls - rs).abs());
    print_table(&g, &S, &path);

    assert_eq!(G as i64, ls + rs);
    assert_eq!(lowest_x, (ls - rs).abs());
}

fn print_table(weights: &Vec<usize>, table: &Vec<Vec<Option<(bool, Direction)>>>, path: &Vec<(usize, usize)>) {
    let imax = table.len();
    let xmax = table[0].len();

    print!("        | ");
    for x in 0..xmax {
        print!("{:02} ", x)
    }
    println!();
    print!("----------");
    for x in 0..xmax {
        print!("---")
    }
    println!();

    for i in 1..imax {
        print!("({:2}) {:02} | ", weights[i - 1], i);
        for x in 0..xmax {
            if let Some(b) = table[i][x] {
                let mut char = if b.0 {
                    match b.1 {
                        Add => { "a" }
                        Sub => { "s" }
                        Both => { "b" }
                        Nowhere => { "x" }
                    }
                } else { "-" }.to_string();

                if path.contains(&(i, x)) {
                    char = char.to_uppercase();
                }
                print!("{}  ", char);
            } else {
                print!("-  ");
            }
        }
        println!();
    }
}
