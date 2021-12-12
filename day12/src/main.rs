use std::env;
use std::fs;
use std::collections::HashMap;

fn dfs<'a>(mappings: &HashMap<&'a str, Vec<&'a str>>, visited: &mut HashMap<&'a str, i32>, current: &'a str, double: &mut Option<&'a str>) -> i32 {
    if visited.get(current) == Some(&2) || (visited.get(current) == Some(&1) && (double != &None || current == "start")) {
        return 0
    } else if current == "end" {
        return 1
    }
    let old_double = double.clone();
    if current == current.to_lowercase() {
        *visited.entry(current).or_default() += 1;
        if visited.get(current) == Some(&2) {
            *double = Some(current);
        }
    }
    let t = mappings.get(current).map(|links| links.iter().map(|link| {
        dfs(mappings, visited, link, double)
    }).sum()).unwrap_or(0);
    *double = old_double;
    visited.entry(current).and_modify(|e| *e -= 1);
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

    println!("{}", dfs(&mappings, &mut HashMap::new(), "start", &mut None));
}
