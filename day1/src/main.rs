use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let nums: Vec<i32> = input.lines().map(|line| line.parse::<i32>().unwrap()).collect();

  let mut larger = 0;

  for i in 1..nums.len() {
    if nums[i] > nums[i-1] {
      larger += 1;
    }
  }
  println!("{}", larger);
}
