use std::env;
use std::fs;

fn calc(lines: &Vec<Vec<char>>, most: bool) -> i32 {
  let mut cum = 0;
  for i in 0..lines[0].len() {
      cum *= 2;
      let zeros = lines.iter().filter(|line| line[i] == '0').count();
      let ones = lines.iter().filter(|line| line[i] == '1').count();
      if (ones > zeros && most) || (ones < zeros && !most) {
          cum += 1;
      }
  }
  cum
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();

  let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

  let gamma = calc(&lines, true);
  let epsilon = calc(&lines, false);
  println!("{}", gamma*epsilon);
}
