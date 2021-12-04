use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();

  let mut x = 0;
  let mut y = 0;
  let mut aim = 0;

  for line in input.lines() {
      let parts: Vec<&str> = line.split(" ").collect();
      let num: i32 = parts[1].parse().unwrap();
      match parts[0] {
        "forward" => {
          x += num;
          y += aim*num;
        },
        "down" => aim += num,
        "up" => aim -= num,
        _ => {},
      }
  }
  println!("{}", x*y);
}
