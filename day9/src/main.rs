use std::env;
use std::fs;

use std::collections::HashMap;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    
    let map: HashMap<i32, HashMap<i32, i32>> = input.lines().enumerate().map(|t| {
        let (i, line_str) = t;
        let line_map: HashMap<i32, i32> = line_str.chars().enumerate().map(|t2| {
            let (j, c) = t2;
            (j as i32, (c as i32 - '0' as i32))
        }).collect();
        (i as i32, line_map)
    }).collect();

    let mut sum = 0;
    for (i, row) in map.iter() {
        for (j, sq) in row.iter() {
            if sq < map.get(&(i-1)).and_then(|row| row.get(j)).unwrap_or(&10) &&
                sq < map.get(&(i+1)).and_then(|row| row.get(j)).unwrap_or(&10) &&
                sq < map.get(i).and_then(|row| row.get(&(j-1))).unwrap_or(&10) &&
                sq < map.get(i).and_then(|row| row.get(&(j+1))).unwrap_or(&10) {
                sum += sq+1;
            }
        }
    }
    println!("{}", sum);
}
