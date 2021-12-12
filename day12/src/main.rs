use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn dfs<'a>(mappings: &HashMap<&'a str, Vec<&'a str>>, visited: &mut HashSet<&'a str>, current: &'a str) -> i32 {
    if visited.contains(current) {
        return 0
    } else if current == "end" {
        return 1
    }
    if current == current.to_lowercase() {
        visited.insert(current);
    }
    let t = mappings.get(current).map(|links| links.iter().map(|link| {
        dfs(mappings, visited, link)
    }).sum()).unwrap_or(0);
    visited.remove(current);
    t
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mut mappings: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split("-").collect();
        mappings.entry(parts[0]).or_default().push(parts[1]);
        mappings.entry(parts[1]).or_default().push(parts[0]);
    }

    println!("{}", dfs(&mappings, &mut HashSet::new(), "start"));
}
