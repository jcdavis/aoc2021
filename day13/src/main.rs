use std::env;
use std::fs;

use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mut lines = input.lines();
    let mut dots: HashSet<(i32, i32)> = HashSet::new();
    loop {
        match lines.next() {
            Some(line) => {
                if line.is_empty() {
                    break;
                }
                let parts: Vec<&str> = line.split(",").collect();
                dots.insert((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
            }
            None => panic!("??")
        }
    }

    for instruction in lines {
        let parts: Vec<&str> = instruction.strip_prefix("fold along ").unwrap().split("=").collect();
        let idx: i32 = parts[1].parse().unwrap();
        for (x,y) in &dots.clone() {
            if parts[0] == "y" && *y > idx {
                let over = *y - idx;
                dots.remove(&(*x, *y));
                dots.insert((*x,idx - over));
            }
            if parts[0] == "x" && *x > idx {
                let over = *x - idx;
                dots.remove(&(*x, *y));
                dots.insert((idx - over, *y));
            }

        }
    }
    let x_max = *dots.iter().map(|(x,_)| x).max().unwrap();
    let y_max = *dots.iter().map(|(_,y)| y).max().unwrap();

    for y in 0..=y_max {
        for x in 0..=x_max {
            if dots.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
