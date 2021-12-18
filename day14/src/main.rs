use std::env;
use std::fs;

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mut lines = input.lines();

    let mut current: HashMap<(char, char), i64> = HashMap::new();
    let mut element_counts: HashMap<char, i64> = HashMap::new();
    let start: Vec<char> = lines.next().unwrap().chars().collect();

    for i in 0..start.len() {
        *element_counts.entry(start[i]).or_default() += 1;
        if i > 0 {
            *current.entry((start[i-1], start[i])).or_default() += 1;
        }
    }
    //empty
    lines.next();

    let mappings: HashMap<(char, char), char> = lines.map(|line| {
        let chars: Vec<char> = line.chars().collect();
        ((chars[0], chars[1]), chars[6])
    }).collect();

    let mut next: HashMap<(char, char), i64> = HashMap::new();

    for _ in 1..=40 {
        for ((left, right), count) in current {
            if let Some(middle) = mappings.get(&(left, right)) {
                *next.entry((left, *middle)).or_default() += count;
                *next.entry((*middle, right)).or_default() += count;
                *element_counts.entry(*middle).or_default() += count;
            }
        }
        current = next.clone();
        next.clear();
    }
    let min = element_counts.values().min().unwrap();
    let max = element_counts.values().max().unwrap();
    println!("{}", max-min);
}
