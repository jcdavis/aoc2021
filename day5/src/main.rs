use std::env;
use std::fs;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;


fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let re = Regex::new(r"(\d+),(\d+)\s+->\s+(\d+),(\d+)").unwrap();
  let lines: Vec<(i32, i32, i32, i32)> = input.lines().map(|line| {
    let c = re.captures(line).unwrap();
    (c[1].parse().unwrap(), c[2].parse().unwrap(), c[3].parse().unwrap(),
     c[4].parse().unwrap())
  }).collect();
  let mut map: HashMap<(i32, i32), i32> = HashMap::new();
  let mut multi_points: HashSet<(i32, i32)> = HashSet::new();

  for line in &lines {
    let (x1, y1, x2, y2) = line;
    if x1 == x2 {
      for y in cmp::min(*y1, *y2)..=cmp::max(*y1, *y2) {
        let prev = *map.get(&(*x1,y)).unwrap_or(&0);
        if prev > 0 {
          multi_points.insert((*x1,y));
        }
        map.insert((*x1,y), prev + 1);
      }
    } else if y1 == y2 {
      for x in cmp::min(*x1, *x2)..=cmp::max(*x1, *x2) {
        let prev = *map.get(&(x,*y1)).unwrap_or(&0);
        if prev > 0 {
          multi_points.insert((x,*y1));
        }
        map.insert((x,*y1), prev + 1);
      }
    } else {
      let dx = if x1 < x2 { 1 } else { -1 };
      let dy = if y1 < y2 { 1 } else { -1 };
      let steps = cmp::max(*x1, *x2) - cmp::min(*x1, *x2) + 1;
      let mut x = *x1;
      let mut y = *y1;
      for _ in 0..steps {
        let prev = *map.get(&(x,y)).unwrap_or(&0);
        if prev > 0 {
          multi_points.insert((x,y));
        }
        map.insert((x,y), prev + 1);
        x += dx;
        y += dy;
      }
    }
  }
  println!("{}", multi_points.len());
}
