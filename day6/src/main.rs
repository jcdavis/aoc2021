use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let fish: Vec<i32> = input.lines().next().expect("1 line").split(",").flat_map(|n| n.parse()).collect();
  let mut states: Vec<i64> = vec![0,0,0,0,0,0,0,0,0];

  for f in fish {
    states[f as usize] += 1;
  }
  for _ in 0..256 {
    let zeros = states.remove(0);
    states[6] += zeros;
    states.push(zeros);
  }
  println!("{}", states.into_iter().sum::<i64>());
}
