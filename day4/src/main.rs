use std::env;
use std::fs;
use std::collections::HashSet;

use regex::Regex;

fn has_bingo(board: &Vec<Vec<i32>>, nums: &HashSet<i32>) -> bool {
  for i in 0..5 {
    let mut bingo = true;
    for j in 0..5 {
      if !nums.contains(&board[i][j]) {
        bingo = false;
        break;
      }
    }
    if bingo {
      return true;
    }
  }
  for j in 0..5 {
    let mut bingo = true;
    for i in 0..5 {
      if !nums.contains(&board[i][j]) {
        bingo = false;
        break;
      }
    }
    if bingo {
      return true;
    }
  }
  false
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = input.lines().collect();
  let sep = Regex::new(r"\s+").unwrap();

  let nums: Vec<i32> = lines[0].split(",").map(|n| n.parse().unwrap()).collect();
  let mut boards: Vec<Vec<Vec<i32>>> = vec!();

  let mut i = 1;
  while i < lines.len() {
    boards.push((i+1..i+6).map(|i| sep.split(lines[i]).flat_map(|n| n.parse()).collect()).collect());
    i += 6;
  }
  let mut set: HashSet<i32> = HashSet::new();

  for num in &nums {
    set.insert(*num);
    boards = boards.into_iter().filter(|board| !has_bingo(&board, &set)).collect();
    if boards.len() == 1 {
      break;
    }
  }

  for num in &nums {
    set.insert(*num);
    if !has_bingo(&boards[0], &set) {
      continue;
    }
    let mut unmarked_sum = 0;
    for row in &boards[0] {
      for col in row {
        if !set.contains(&col) {
          unmarked_sum += col;
        }
      }
    }
    println!("{}", num*unmarked_sum);
    return;
  }
}
