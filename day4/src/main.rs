use std::env;
use std::fs;
use std::collections::HashSet;

use regex::Regex;

fn has_bingo(board: &Vec<Vec<i32>>, nums: &HashSet<i32>) -> Option<i32> {
  if nums.contains(&board[0][0]) &&
     nums.contains(&board[1][1]) &&
     nums.contains(&board[2][2]) &&
     nums.contains(&board[3][3]) &&
     nums.contains(&board[4][4]) {
    return Some(board[0][0] + board[1][1] + board[2][2] + board[3][3] + board[4][4]);
  }
  if nums.contains(&board[0][4]) &&
     nums.contains(&board[1][3]) &&
     nums.contains(&board[2][2]) &&
     nums.contains(&board[3][1]) &&
     nums.contains(&board[4][0]) {
    return Some(board[0][4] + board[1][3] + board[2][2] + board[3][1] + board[4][0]);
  }
  for i in 0..5 {
    let mut sum = 0;
    for j in 0..5 {
      if !nums.contains(&board[i][j]) {
        sum = -1;
        break;
      }
      sum += board[i][j];
    }
    if sum > 0 {
      return Some(sum);
    }
  }
  for j in 0..5 {
    let mut sum = 0;
    for i in 0..5 {
      if !nums.contains(&board[i][j]) {
        sum = -1;
        break;
      }
      sum += board[i][j];
    }
    if sum > 0 {
      return Some(sum);
    }
  }
  None
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
    for board in &boards {
        if let Some(bingo_sum) = has_bingo(&board, &set) {
            let mut unmarked_sum = 0;
            for row in board {
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
  }
}
