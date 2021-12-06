use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let mut fish: Vec<i32> = input.lines().next().expect("1 line").split(",").flat_map(|n| n.parse()).collect();

  for _ in 0..80 {
    let end = fish.len();
    for i in 0..end {
      if fish[i] == 0 {
        fish[i] = 6;
        fish.push(8);
      } else {
        fish[i] -= 1;
      }
    }
  }
  println!("{}", fish.len());
}
