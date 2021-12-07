use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();

  let nums: Vec<i32> = input.lines().next().expect("1 line").split(",").flat_map(|n| n.parse()).collect();

  let min: i32 = *nums.iter().min().expect("nonzero");
  let max: i32 = *nums.iter().max().expect("nonzero");

  let mut min_cost = 1000000000;
  let mut min_idx = -1;
  for i in min..=max {
    let cost: i32 = nums.iter().map(|n| {
      let d = (*n-i).abs();
      d*(d+1)/2
    }).sum();
    if cost < min_cost {
      min_cost = cost;
      min_idx = i;
    }
  }
  println!("{} {}", min_idx, min_cost);
}
