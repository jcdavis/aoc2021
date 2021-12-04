use std::env;
use std::fs;

fn calc(lines: &Vec<Vec<char>>, most: bool) -> i32 {
  let mut remaining: Vec<&Vec<char>> = lines.iter().collect();
  let mut i = 0;
  while remaining.len() > 1 {
      let zeros = remaining.iter().filter(|line| line[i] == '0').count();
      let ones = remaining.iter().filter(|line| line[i] == '1').count();
      let to_filter: char = if (ones >= zeros && most) || (ones < zeros && !most) {
        '1'
      } else {
        '0'
      };
      remaining = remaining.into_iter().filter(|line| line[i] == to_filter).collect();
      i += 1;
  }
  let mut cum = 0;
  for i in 0..remaining[0].len() {
      cum *= 2;
      if remaining[0][i] == '1' {
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
