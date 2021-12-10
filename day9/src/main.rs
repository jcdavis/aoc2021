use std::env;
use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

fn recurse(map: &HashMap<i32, HashMap<i32, i32>>, seen: &mut HashSet<(i32, i32)>, i: i32, j: i32, prev: i32) {
    if seen.contains(&(i,j)) {
        return;
    }
    match map.get(&i).and_then(|row| row.get(&j)) {
        Some(cell) => {
            if *cell < 9 && *cell > prev {
                seen.insert((i,j));
                recurse(map, seen, i-1, j, *cell);
                recurse(map, seen, i+1, j, *cell);
                recurse(map, seen, i, j-1, *cell);
                recurse(map, seen, i, j+1, *cell);
            }
        }
        _ => {}
    }
}

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

    let mut highest = vec!(0,0,0);
    for (i, row) in map.iter() {
        for (j, sq) in row.iter() {
            if sq < map.get(&(i-1)).and_then(|row| row.get(j)).unwrap_or(&10) &&
                sq < map.get(&(i+1)).and_then(|row| row.get(j)).unwrap_or(&10) &&
                sq < map.get(i).and_then(|row| row.get(&(j-1))).unwrap_or(&10) &&
                sq < map.get(i).and_then(|row| row.get(&(j+1))).unwrap_or(&10) {
                let mut seen: HashSet<(i32, i32)> = HashSet::new();
                recurse(&map, &mut seen, *i, *j, -1);
                highest.push(seen.len());
                highest.sort();
                highest.remove(0);
            }
        }
    }
    println!("{}", highest[0] * highest[1] * highest[2]);
}
